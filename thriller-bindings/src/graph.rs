use pyo3::prelude::*;
use pyo3::types::PyList;

use thriller_core::{
    AccessMap, Gemm, MemoryLevel, Task, ThrillerEdge, ThrillerGraph, ThrillerNode,
    ThrillerNodeInner,
};

use crate::buffer::PyBuffer;

use std::{cell::RefCell, rc::Rc};

#[pyclass]
pub enum PyMemoryLevel {
    Register,
    Shared,
    Global,
}

#[pyclass(unsendable)]
pub struct PyGraph(pub Rc<RefCell<ThrillerGraph>>);

#[pymethods]
impl PyGraph {
    #[new]
    fn new(mem_level: &PyMemoryLevel) -> PyGraph {
        let mem_level = match mem_level {
            PyMemoryLevel::Register => MemoryLevel::Register,
            PyMemoryLevel::Shared => MemoryLevel::Shared,
            PyMemoryLevel::Global => MemoryLevel::Global,
        };

        PyGraph(Rc::new(RefCell::new(ThrillerGraph::new(mem_level))))
    }

    fn add_nodes(&mut self, nodes: &Bound<'_, PyList>) -> PyResult<()> {
        let nodes = nodes
            .into_iter()
            .map(|node| {
                // TODO(KuangjuX): fix `unwarp`.
                let node = node.extract::<PyRef<PyNode>>().unwrap();
                Rc::clone(&node.0)
            })
            .collect::<Vec<_>>();

        self.0.borrow_mut().add_nodes(nodes);
        Ok(())
    }

    fn add_edges(&mut self, edges: &Bound<'_, PyList>) -> PyResult<()> {
        let edges = edges
            .into_iter()
            .map(|edge| {
                // TODO(KuangjuX): fix `unwarp`.
                let edge = edge.extract::<PyRef<PyEdge>>().unwrap();
                Rc::clone(&edge.0)
            })
            .collect::<Vec<_>>();

        self.0.borrow_mut().add_edges(edges);
        Ok(())
    }

    fn connect(&mut self) {
        self.0.borrow_mut().connect();
    }

    fn codegen(&self) -> PyResult<String> {
        self.0
            .borrow()
            .emit()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))
    }
}

#[pyclass(unsendable)]
pub struct PyNode(pub Rc<RefCell<ThrillerNode>>);

#[pymethods]
impl PyNode {
    #[new]
    fn buffer(buf: &PyBuffer) -> Self {
        let node = ThrillerNode::new(thriller_core::ThrillerNodeInner::Buffer(Rc::clone(&buf.0)));
        PyNode(Rc::new(RefCell::new(node)))
    }

    fn gemm(a: PyRef<PyNode>, b: PyRef<PyNode>, c: PyRef<PyNode>) -> Self {
        let access_map = AccessMap::new(0, vec![]);

        let node_a = Rc::clone(&a.0);
        let node_b = Rc::clone(&b.0);
        let node_c = Rc::clone(&c.0);

        let gemm = Gemm::new(vec![node_a, node_b], node_c, Rc::new(access_map));

        let node = ThrillerNode::new(ThrillerNodeInner::Op(Box::new(gemm)));

        PyNode(Rc::new(RefCell::new(node)))
    }

    fn codegen(&self) -> PyResult<String> {
        let node = self.0.borrow();
        node.emit()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))
    }
}

#[pyclass(unsendable)]
pub struct PyEdge(pub Rc<ThrillerEdge>);

#[pymethods]
impl PyEdge {
    #[new]
    fn new(src: PyRef<PyNode>, dst: PyRef<PyNode>) -> Self {
        let src = Rc::clone(&src.0);
        let dst = Rc::clone(&dst.0);
        let edge = ThrillerEdge::new(src, dst);
        PyEdge(Rc::new(edge))
    }
}

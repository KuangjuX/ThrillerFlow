//! ThrillerFlow is a DataFlow Analyise and Codegen Framework written in Rust.

#![deny(warnings)]
#![deny(missing_docs)]
mod access;
mod buffer;
mod dataflow;
mod engine;
mod error;
mod id;
mod task;
mod var;

pub use access::{AccessMap, AccessMatrix, AccessOffset};
pub use buffer::Buffer;
pub use dataflow::{
    AttachedEdge, BlockType, ThrillerBlock, ThrillerEdge, ThrillerGraph, ThrillerNode,
    ThrillerNodeInner,
};
pub use engine::ThrillerEngine;
pub use error::{ThrillerError, ThrillerResult};
pub use task::{Gemm, Task};
pub use var::{IterationBound, IterationVar, RegularVar, Var};

use id::ID_COUNTER;

/// GPU Memory Level.
#[derive(Default, Clone, Copy)]
pub enum MemoryLevel {
    /// Register File
    #[default]
    Register,
    /// Shared Memory
    Shared,
    /// Global Memory
    Global,
}

/// Initialize the ThrillerFlow framework.
pub fn initialize() {
    let id_counter = id::IdCounter::new();
    unsafe {
        id::ID_COUNTER.get_or_init(|| id_counter);
    }
}

/// Return the next unique ID.
pub fn next_id() -> usize {
    unsafe { ID_COUNTER.get_mut().unwrap().next() }
}

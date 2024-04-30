//! ThrillerFlow is a DataFlow Analyise and Codegen Framework written in Rust.

#![deny(warnings)]
#![deny(missing_docs)]
mod access;
mod buffer;
mod dataflow;
mod id;
mod task;

pub use access::AccessMap;
pub use buffer::Buffer;
pub use dataflow::{ThrillerEdge, ThrillerGraph, ThrillerNode};

/// GPU Memory Level.
#[derive(Default)]
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

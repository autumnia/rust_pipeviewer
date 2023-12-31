//! pipeline library documentation

pub mod args;
pub mod read;
pub mod stats;
pub mod write;

const CHUNK_SIZE: usize = 16 * 1024;

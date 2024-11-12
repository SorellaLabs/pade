mod decode;
mod encode;
mod header;
mod primitives;
// Re-export bitvec so our macro crate can rely on it
pub use bitvec;
pub use decode::*;
pub use encode::*;
pub use header::HeaderBits;

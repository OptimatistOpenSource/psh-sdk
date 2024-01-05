#[deny(clippy::all)]
pub(crate) mod bindings;

pub mod file;
mod intrinsics;
pub mod perf;

pub use intrinsics::*;

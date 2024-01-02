#[deny(clippy::all)]
pub(crate) mod bindings;

pub mod file;
mod intrinsics;

pub use intrinsics::*;

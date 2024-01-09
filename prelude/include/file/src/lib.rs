#![no_std]
#![cfg(target_arch = "wasm32")]

extern crate alloc;

#[deny(clippy::all)]
pub(crate) mod bindings;

mod wrapped;

pub use wrapped::*;

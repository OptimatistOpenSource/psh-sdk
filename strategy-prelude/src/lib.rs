#![no_std]
#![cfg(target_arch = "wasm32")]

extern crate alloc;

pub mod op;
mod intrinsics;

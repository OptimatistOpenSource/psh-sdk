#![cfg(target_arch = "wasm32")]
#![no_std]

use prelude::intrinsics;
use prelude::proc_macros::main;
use profiling::prelude;

#[main]
fn main() {
    intrinsics::log("0");
    intrinsics::log("1");
    intrinsics::log("2");
}

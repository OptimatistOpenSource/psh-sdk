#![cfg(target_arch = "wasm32")]
#![no_std]

use prelude::intrinsics;
use prelude::proc_macros::main;
use profiling::prelude;

#[main]
fn main() {
    intrinsics::log("0");
    intrinsics::exit();
    intrinsics::log("1");
}

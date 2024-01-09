#![cfg(target_arch = "wasm32")]
#![no_std]

use profiling::prelude;
use prelude::intrinsics;

#[profiling::main]
fn main() {
    intrinsics::log("0");
    intrinsics::log("1");
    intrinsics::log("2");
}

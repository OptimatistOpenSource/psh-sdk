#![cfg(target_arch = "wasm32")]
#![no_std]

use profiling::prelude;
use prelude::intrinsics;

#[profiling::main]
fn main() {
    intrinsics::log("0");
    intrinsics::exit();
    intrinsics::log("1");
}

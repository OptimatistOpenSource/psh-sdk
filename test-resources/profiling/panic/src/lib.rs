#![cfg(target_arch = "wasm32")]
#![no_std]

use profiling::prelude;
use prelude::intrinsics;

#[profiling::main]
fn main() {
    intrinsics::log("0");
    panic!("oops");
    intrinsics::log("1");
}

#![cfg(target_arch = "wasm32")]
#![no_std]

use profiling::op;

#[profiling::main]
fn main() {
    op::log("0");
    op::log("1");
    op::log("2");
}

#![cfg(target_arch = "wasm32")]
#![no_std]

use strategy::op;

#[strategy::main]
fn main() {
    op::log("Hello world!");
}

#![allow(dead_code)]

use core::borrow::Borrow;

#[deny(clippy::all)]
pub mod bindings;
pub mod file;

#[inline]
pub fn log(info: impl Borrow<str>) {
    bindings::op::log(info.borrow())
}

#[inline]
pub fn exit() {
    bindings::op::exit()
}

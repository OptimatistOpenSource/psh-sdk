use crate::op::bindings;
use core::borrow::Borrow;

#[inline]
pub fn log(info: impl Borrow<str>) {
    let info = info.borrow();
    let info_ptr = info.as_ptr();
    let info_len = info.len();
    bindings::op::log(info_ptr as _, info_len as _)
}

#[inline]
pub fn exit() {
    bindings::op::exit()
}

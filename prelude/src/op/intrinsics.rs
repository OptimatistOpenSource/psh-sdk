use crate::op::bindings;
use core::borrow::Borrow;

pub fn log(info: impl Borrow<str>) {
    let info = info.borrow();
    let info_ptr = info.as_ptr();
    let info_len = info.len();
    bindings::op::log(info_ptr as _, info_len as _)
}

pub fn log_err(info: impl Borrow<str>) {
    let info = info.borrow();
    let info_ptr = info.as_ptr();
    let info_len = info.len();
    bindings::op::log_err(info_ptr as _, info_len as _)
}

#[inline]
pub fn exit() {
    bindings::op::exit()
}

pub fn drop_resource(id: u32) -> bool {
    match bindings::op::drop_resource(id) {
        0 => true,
        1 => false,
        _ => unreachable!(),
    }
}

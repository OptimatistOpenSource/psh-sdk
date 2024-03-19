use crate::bindings;
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

pub fn drop_resource(rid: u32) {
    bindings::op::drop_resource(rid)
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    let info = alloc::format!("Profiling panic: \n{}", info);
    log_err(info);
    exit();

    loop {}
}

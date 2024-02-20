#[cfg(test)]
mod tests;

use crate::infra::wasm::get_str;
use crate::profiling::runtime::Data;
use std::ops::Not;
use wasmtime::Caller;

pub fn log(mut caller: Caller<Data>, info_vm_ptr: u32, info_len: u32) {
    let caller = &mut caller;

    let info = unsafe { get_str(caller, info_vm_ptr, info_len).to_string() };
    let out = caller.data_mut().out();

    out(info.as_str());
}

pub fn log_err(mut caller: Caller<Data>, info_vm_ptr: u32, info_len: u32) {
    let caller = &mut caller;

    let info = unsafe { get_str(caller, info_vm_ptr, info_len).to_string() };
    let err = caller.data_mut().err();

    err(info.as_str());
}

pub fn exit(caller: Caller<Data>) {
    caller.engine().increment_epoch();
}

pub fn drop_resource(mut caller: Caller<Data>, id: u32) {
    let data = caller.data_mut();

    if data.drop_resource(id).not() {
        data.err()("Failed to drop resource");
        exit(caller)
    }
}

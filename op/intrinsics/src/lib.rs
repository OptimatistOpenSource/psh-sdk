use std::ops::Not;

use profiling_runtime::{infra::str::StrExt, infra::wasm::get_mem, profiling::runtime::Data};
use wasmtime::Caller;

#[cfg(test)]
mod tests;

pub fn log(mut caller: Caller<Data>, info_vm_ptr: u32, info_len: u32) {
    let caller = &mut caller;
    let mem = get_mem(caller);

    let info = unsafe { <&str>::from_wasm_mem(mem, info_vm_ptr, info_len).to_string() };
    let out = caller.data_mut().out();

    out(info.as_str());
}

pub fn log_err(mut caller: Caller<Data>, info_vm_ptr: u32, info_len: u32) {
    let caller = &mut caller;
    let mem = get_mem(caller);

    let info = unsafe { <&str>::from_wasm_mem(mem, info_vm_ptr, info_len).to_string() };
    let err = caller.data_mut().err();

    err(info.as_str());
}

pub fn exit(caller: Caller<Data>) {
    caller.engine().increment_epoch();
}

pub fn drop_resource(mut caller: Caller<Data>, rid: u32) {
    let data = caller.data_mut();

    if data.drop_resource(rid).not() {
        data.err()("Failed to drop resource");
        exit(caller)
    }
}

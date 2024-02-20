#[cfg(test)]
mod tests;

use crate::infra::wasm::get_str;
use crate::profiling::runtime::Data;
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

pub fn exit(c: Caller<Data>) {
    c.engine().increment_epoch();
}

pub fn drop_resource(mut c: Caller<Data>, id: u32) -> u32 {
    c.data_mut().drop_resource(id) as u32
}

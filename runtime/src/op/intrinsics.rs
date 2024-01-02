use crate::infra::wasm::get_str;
use crate::op;
use crate::profiling::engine::Data;
use wasmtime::Caller;

pub fn log(mut caller: Caller<Data>, info_vm_ptr: u32, info_len: u32) {
    let caller = &mut caller;
    unsafe {
        let info = get_str(caller, info_vm_ptr, info_len).to_string();
        op::raw::intrinsics::log(info, caller.data_mut())
    }
}

pub fn exit(c: Caller<Data>) {
    c.engine().increment_epoch();
    op::raw::intrinsics::exit()
}

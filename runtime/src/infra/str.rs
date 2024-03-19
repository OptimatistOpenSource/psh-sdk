use std::slice;
use crate::infra::wasm::to_host_ptr;

pub trait StrExt {
    unsafe fn from_wasm_mem(mem: &[u8], vm_ptr: u32, len: u32) -> &str;
}

impl StrExt for &str {
    unsafe fn from_wasm_mem(mem: &[u8], vm_ptr: u32, len: u32) -> &str {
        let ptr = to_host_ptr(mem, vm_ptr);
        let slice = slice::from_raw_parts(ptr as _, len as _);
        std::str::from_utf8(slice).unwrap()
    }
}

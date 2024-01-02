use std::slice;
use wasmtime::{AsContext, AsContextMut, Caller, Memory, Val};

pub fn get_mem<T>(caller: &mut Caller<T>) -> Memory {
    caller.get_export("memory").unwrap().into_memory().unwrap()
}

pub unsafe fn vm_alloc<T>(caller: &mut Caller<T>, size: u32, align: u32) -> u32 {
    // alloc: u32 -> u32 -> u32
    let alloc = caller.get_export("alloc").unwrap().into_func().unwrap();
    let params = [Val::I32(size as _), Val::I32(align as _)];
    let mut results = [Val::I32(0)];
    alloc
        .call(caller.as_context_mut(), &params, &mut results)
        .unwrap();
    results[0].i32().unwrap() as _
}

pub unsafe fn vm_dealloc<T>(caller: &mut Caller<T>, vm_ptr: u32, size: u32, align: u32) {
    // dealloc: u32 -> u32 -> u32 -> ()
    let dealloc = caller.get_export("dealloc").unwrap().into_func().unwrap();
    let params = [
        Val::I32(vm_ptr as _),
        Val::I32(size as _),
        Val::I32(align as _),
    ];
    dealloc
        .call(caller.as_context_mut(), &params, &mut [])
        .unwrap();
}

pub unsafe fn to_host_ptr<T>(caller: &mut Caller<T>, vm_ptr: u32) -> *mut u8 {
    get_mem(caller)
        .data_ptr(caller.as_context())
        .add(vm_ptr as _)
}

pub unsafe fn get_str<'t, T>(caller: &'t mut Caller<T>, vm_ptr: u32, len: u32) -> &'t str {
    let ptr = to_host_ptr(caller, vm_ptr);
    let slice = slice::from_raw_parts(ptr as _, len as _);
    std::str::from_utf8(slice).unwrap()
}

pub unsafe fn copy_to_vm<T, V: ?Sized>(caller: &mut Caller<T>, val: &V) -> u32 {
    let size = std::mem::size_of_val(val);
    let align = std::mem::align_of_val(val);
    let dst_vm_ptr = vm_alloc(caller, size as _, align as _);

    let dst = to_host_ptr(caller, dst_vm_ptr);
    let src = val as *const _ as *const _;
    std::ptr::copy_nonoverlapping(src, dst, size);

    dst_vm_ptr
}

pub unsafe fn move_to_vm<T, V>(caller: &mut Caller<T>, val: V) -> u32 {
    let size = std::mem::size_of::<V>();
    let align = std::mem::align_of::<V>();
    let dst_vm_ptr = vm_alloc(caller, size as _, align as _);

    let dst = to_host_ptr(caller, dst_vm_ptr);
    let src = &val as *const _ as *const _;
    std::ptr::copy_nonoverlapping(src, dst, size);

    dst_vm_ptr
}

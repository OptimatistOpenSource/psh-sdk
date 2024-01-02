use crate::infra::wasm::{copy_to_vm, get_str, to_host_ptr};
use crate::op;
use crate::profiling::engine::Data;
use wasmtime::Caller;

pub fn is_exist(mut caller: Caller<Data>, path_vm_ptr: u32, path_len: u32) -> u32 {
    let caller = &mut caller;
    unsafe {
        let path = get_str(caller, path_vm_ptr, path_len);
        op::raw::file::is_exist(path) as _
    }
}

pub fn read(mut caller: Caller<Data>, ret_area_vm_ptr: u32, path_vm_ptr: u32, path_len: u32) {
    let caller = &mut caller;
    unsafe {
        let ret_area = &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]);
        let path = get_str(caller, path_vm_ptr, path_len);
        match op::raw::file::read(path) {
            Ok(contents) => {
                let vm_ptr = copy_to_vm(caller, contents.as_str());
                *ret_area = [1, vm_ptr, contents.len() as _];
            }
            Err(e) => {
                let vm_ptr = copy_to_vm(caller, e.as_str());
                *ret_area = [0, vm_ptr, e.len() as _];
            }
        }
    }
}

pub fn write(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    path_vm_ptr: u32,
    path_len: u32,
    contents_vm_ptr: u32,
    contents_len: u32,
) {
    let caller = &mut caller;
    unsafe {
        let ret_area = &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]);
        let path = get_str(caller, path_vm_ptr, path_len).to_string();
        let contents = get_str(caller, contents_vm_ptr, contents_len);
        match op::raw::file::write(&path, contents) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let vm_ptr = copy_to_vm(caller, e.as_str());
                *ret_area = [0, vm_ptr, e.len() as _];
            }
        }
    }
}

pub fn append(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    path_vm_ptr: u32,
    path_len: u32,
    contents_vm_ptr: u32,
    contents_len: u32,
) {
    let caller = &mut caller;
    unsafe {
        let ret_area = &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]);
        let path = get_str(caller, path_vm_ptr, path_len).to_string();
        let contents = get_str(caller, contents_vm_ptr, contents_len);
        match op::raw::file::append(&path, contents) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let vm_ptr = copy_to_vm(caller, e.as_str());
                *ret_area = [0, vm_ptr, e.len() as _];
            }
        }
    }
}

pub fn remove_file(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    path_vm_ptr: u32,
    path_len: u32,
) {
    let caller = &mut caller;
    unsafe {
        let ret_area = &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]);
        let path = get_str(caller, path_vm_ptr, path_len).to_string();
        match op::raw::file::remove_file(&path) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let vm_ptr = copy_to_vm(caller, e.as_str());
                *ret_area = [0, vm_ptr, e.len() as _];
            }
        }
    }
}

pub fn create_dir(mut caller: Caller<Data>, ret_area_vm_ptr: u32, path_vm_ptr: u32, path_len: u32) {
    let caller = &mut caller;
    unsafe {
        let ret_area = &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]);
        let path = get_str(caller, path_vm_ptr, path_len).to_string();
        match op::raw::file::create_dir(&path) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let vm_ptr = copy_to_vm(caller, e.as_str());
                *ret_area = [0, vm_ptr, e.len() as _];
            }
        }
    }
}

pub fn remove_dir(mut caller: Caller<Data>, ret_area_vm_ptr: u32, path_vm_ptr: u32, path_len: u32) {
    let caller = &mut caller;
    unsafe {
        let ret_area = &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]);
        let path = get_str(caller, path_vm_ptr, path_len).to_string();
        match op::raw::file::remove_dir(&path) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let vm_ptr = copy_to_vm(caller, e.as_str());
                *ret_area = [0, vm_ptr, e.len() as _];
            }
        }
    }
}

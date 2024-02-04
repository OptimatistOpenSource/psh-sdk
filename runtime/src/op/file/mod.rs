mod raw;
#[cfg(test)]
mod tests;

use crate::infra::wasm::{copy_to_vm, get_str, to_host_ptr, vm_alloc};
use crate::profiling::runtime::Data;
use std::fs::File;
use std::io::Read;
use std::slice;
use wasmtime::Caller;

pub fn is_exist(mut caller: Caller<Data>, path_vm_ptr: u32, path_len: u32) -> u32 {
    let caller = &mut caller;
    unsafe {
        let path = get_str(caller, path_vm_ptr, path_len);
        raw::is_exist(path) as _
    }
}

pub fn read(mut caller: Caller<Data>, ret_area_vm_ptr: u32, path_vm_ptr: u32, path_len: u32) {
    let caller = &mut caller;

    let ret_area_ptr = unsafe { to_host_ptr(caller, ret_area_vm_ptr) } as *mut [u32; 3];
    let ret_area = unsafe { &mut *ret_area_ptr };

    let path = unsafe { get_str(caller, path_vm_ptr, path_len) };

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            let e = e.to_string();
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
            return;
        }
    };
    let file_len = match file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            let e = e.to_string();
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
            return;
        }
    };
    let buf_vm_ptr = unsafe { vm_alloc(caller, file_len as _, 1) };
    let buf_ptr = unsafe {
        let buf_ptr = to_host_ptr(caller, buf_vm_ptr);
        slice::from_raw_parts_mut(buf_ptr, file_len as _)
    };
    let buf = &mut *buf_ptr;

    match file.read_exact(buf) {
        Ok(_) => {
            *ret_area = [1, buf_vm_ptr, file_len as _];
        }
        Err(e) => {
            let e = e.to_string();
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
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
        match raw::write(&path, contents) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let e = e.to_string();
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
        match raw::append(&path, contents) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let e = e.to_string();
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
        match raw::remove_file(&path) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let e = e.to_string();
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
        match raw::create_dir(&path) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let e = e.to_string();
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
        match raw::remove_dir(&path) {
            Ok(_) => {
                ret_area[0] = 1;
            }
            Err(e) => {
                let e = e.to_string();
                let vm_ptr = copy_to_vm(caller, e.as_str());
                *ret_area = [0, vm_ptr, e.len() as _];
            }
        }
    }
}

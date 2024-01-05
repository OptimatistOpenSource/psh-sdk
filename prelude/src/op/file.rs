use crate::op::bindings::op::*;
use alloc::string::String;
use core::borrow::Borrow;
use core::mem::MaybeUninit;

pub fn is_exist(path: impl Borrow<str>) -> bool {
    let path = path.borrow();
    let path_ptr = path.as_ptr() as _;
    let path_len = path.len() as _;

    match file_is_exist(path_ptr, path_len) {
        1 => true,
        0 => false,
        _ => unreachable!(),
    }
}

pub fn read(path: impl Borrow<str>) -> Result<String, String> {
    #[allow(invalid_value)]
    let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
    let ret_area_ptr = ret_area.as_ptr() as _;
    let path = path.borrow();
    let path_ptr = path.as_ptr() as _;
    let path_len = path.len() as _;
    file_read(ret_area_ptr, path_ptr, path_len);

    let [is_ok, ptr, len] = ret_area;
    match is_ok {
        1 => Ok(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
        0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
        _ => unreachable!(),
    }
}

pub fn write(path: impl Borrow<str>, contents: impl Borrow<str>) -> Result<(), String> {
    #[allow(invalid_value)]
    let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
    let ret_area_ptr = ret_area.as_ptr() as _;
    let path = path.borrow();
    let path_ptr = path.as_ptr() as _;
    let path_len = path.len() as _;
    let contents = contents.borrow();
    let contents_ptr = contents.as_ptr() as _;
    let contents_len = contents.len() as _;
    file_write(ret_area_ptr, path_ptr, path_len, contents_ptr, contents_len);

    let [is_ok, ptr, len] = ret_area;
    match is_ok {
        1 => Ok(()),
        0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
        _ => unreachable!(),
    }
}

pub fn append(path: impl Borrow<str>, contents: impl Borrow<str>) -> Result<(), String> {
    #[allow(invalid_value)]
    let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
    let ret_area_ptr = ret_area.as_ptr() as _;
    let path = path.borrow();
    let path_ptr = path.as_ptr() as _;
    let path_len = path.len() as _;
    let contents = contents.borrow();
    let contents_ptr = contents.as_ptr() as _;
    let contents_len = contents.len() as _;
    file_append(ret_area_ptr, path_ptr, path_len, contents_ptr, contents_len);

    let [is_ok, ptr, len] = ret_area;
    match is_ok {
        1 => Ok(()),
        0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
        _ => unreachable!(),
    }
}

pub fn remove_file(path: impl Borrow<str>) -> Result<(), String> {
    #[allow(invalid_value)]
    let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
    let ret_area_ptr = ret_area.as_ptr() as _;
    let path = path.borrow();
    let path_ptr = path.as_ptr() as _;
    let path_len = path.len() as _;
    file_remove_file(ret_area_ptr, path_ptr, path_len);

    let [is_ok, ptr, len] = ret_area;
    match is_ok {
        1 => Ok(()),
        0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
        _ => unreachable!(),
    }
}

pub fn create_dir(path: impl Borrow<str>) -> Result<(), String> {
    #[allow(invalid_value)]
    let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
    let ret_area_ptr = ret_area.as_ptr() as _;
    let path = path.borrow();
    let path_ptr = path.as_ptr() as _;
    let path_len = path.len() as _;
    file_create_dir(ret_area_ptr, path_ptr, path_len);

    let [is_ok, ptr, len] = ret_area;
    match is_ok {
        1 => Ok(()),
        0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
        _ => unreachable!(),
    }
}

pub fn remove_dir(path: impl Borrow<str>) -> Result<(), String> {
    #[allow(invalid_value)]
    let ret_area = unsafe { MaybeUninit::<[u32; 3]>::uninit().assume_init() };
    let ret_area_ptr = ret_area.as_ptr() as _;
    let path = path.borrow();
    let path_ptr = path.as_ptr() as _;
    let path_len = path.len() as _;
    file_remove_dir(ret_area_ptr, path_ptr, path_len);

    let [is_ok, ptr, len] = ret_area;
    match is_ok {
        1 => Ok(()),
        0 => Err(unsafe { String::from_raw_parts(ptr as _, len as _, len as _) }),
        _ => unreachable!(),
    }
}

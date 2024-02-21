#![cfg(target_arch = "wasm32")]
#![no_std]

extern crate alloc;

use core::ops::Not;
use prelude::file;
use prelude::proc_macros::main;
use profiling::prelude;

#[main]
fn main() {
    let file_path = "../test-resources/tmp/profiling-test-file";

    /*
    test:
        op::file::write
        op::file::append
        op::file::read
    */
    file::write(file_path, "0").unwrap();
    file::append(file_path, "1").unwrap();
    let contents = file::read(file_path).unwrap();
    assert_eq!(contents, "01");

    /*
    test:
        op::file::exists
        op::file::remove_file
    */
    let exists = file::exists(file_path);
    assert!(exists);
    file::remove_file(file_path).unwrap();
    let exists = file::exists(file_path);
    assert!(exists.not());

    /*
    test:
        op::file::exists
        op::file::create_dir
        op::file::remove_dir
    */
    file::create_dir(file_path).unwrap();
    let exists = file::exists(file_path);
    assert!(exists);
    file::remove_dir(file_path).unwrap();
    let exists = file::exists(file_path);
    assert!(exists.not());
}

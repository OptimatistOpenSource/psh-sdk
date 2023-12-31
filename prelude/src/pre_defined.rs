extern crate alloc;
use core::alloc::Layout;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[export_name = "alloc"]
unsafe extern "C" fn alloc(size: i32, align: i32) -> i32 {
    use alloc::alloc::alloc;
    let layout = Layout::from_size_align(size as _, align as _).unwrap();
    let ptr = unsafe { alloc(layout) };
    ptr as _
}

#[export_name = "dealloc"]
unsafe extern "C" fn dealloc(ptr: i32, size: i32, align: i32) {
    use alloc::alloc::dealloc;
    let layout = Layout::from_size_align(size as _, align as _).unwrap();
    unsafe { dealloc(ptr as _, layout) }
}

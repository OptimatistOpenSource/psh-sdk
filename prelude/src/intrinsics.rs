#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    extern crate alloc;
    use crate::op;
    use alloc::format;
    use alloc::string::ToString;

    let info = format!("profiling panic: {}", info.to_string());
    op::log(info);
    op::exit();

    loop {}
}

#[export_name = "alloc"]
unsafe extern "C" fn alloc(size: i32, align: i32) -> i32 {
    extern crate alloc;
    use alloc::alloc::alloc;
    use core::alloc::Layout;

    let layout = Layout::from_size_align(size as _, align as _).unwrap();
    let ptr = unsafe { alloc(layout) };
    ptr as _
}

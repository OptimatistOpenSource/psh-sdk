#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    extern crate alloc;
    use crate::op;
    use alloc::format;
    use alloc::string::ToString;

    let info = format!("strategy panic: {}", info.to_string());
    op::log(info);
    op::exit();

    loop {}
}

/*pub(crate) struct Stub;

impl crate::op::bindings::Guest for Stub {
    #[inline]
    fn main() {
        unsafe {}
    }

    #[inline]
    fn alloc(size: u32, align: u32) -> u32 {
        use alloc::alloc::alloc;
        use core::alloc::Layout;

        let layout = Layout::from_size_align(size as _, align as _).unwrap();
        let ptr = unsafe { alloc(layout) };
        ptr as _
    }
}*/
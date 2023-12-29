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

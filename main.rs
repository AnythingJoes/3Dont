#![no_main]

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn hello_sum(x: i32, y: i32) -> i32 {
    x + y
}

#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_begin_unwind() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn eh_personality() {}

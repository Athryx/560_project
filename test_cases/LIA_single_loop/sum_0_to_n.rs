#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

// -------------------------------------
// Sum of first n integers
// -------------------------------------
#[no_mangle]
pub extern "C" fn sum_range(n: i32) -> i32 {
    let mut total = 0;
    let mut i = 0;

    while i < n {
        total = total + i;
        i += 1;
    }

    total
}

#[no_mangle]
pub extern "C" fn main(n: i32) -> i32 {
    let _ = sum_range(n);
    0
}

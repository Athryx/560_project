#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn forward_backward(a: i32) -> i32 {
    let mut total = 0;

    // forward accumulation
    let mut i = 0;
    while i < a {
        total = total + i;
        i += 1;
    }

    // backward accumulation
    let mut j = a;
    while j > 0 {
        total = total + (j - 1);
        j -= 1;
    }

    total
}

#[no_mangle]
pub extern "C" fn main(a: i32) -> i32 {
    let _ = forward_backward(a);
    0
}

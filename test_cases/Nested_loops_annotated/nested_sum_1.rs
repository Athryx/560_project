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
// Your logic function (LIA + nested loops)
// -------------------------------------

// requires: a >= 0, b >= 0
// ensures: returns a * b

#[no_mangle]
pub extern "C" fn nested_sum(a: i32, b: i32) -> i32 {
    let mut total = 0;

    let mut i = 0;
    while i < a {
        let mut j = 0;
        while j < b {
            total += 1;
            j += 1;
        }
        i += 1;
    }

    total
}

// -------------------------------------
// Minimal main that calls your function
// -------------------------------------
#[no_mangle]
pub extern "C" fn main(a: i32, b: i32) -> i32 {
    let r = nested_sum(a, b);
    0
}

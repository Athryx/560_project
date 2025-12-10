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

// requires (a >= 0 && b >= 0)
// ensures (result == ((a * (a - 1)) / 2) + (b * b))
pub extern "C" fn two_sums(a: i32, b: i32) -> i32 {
    let mut s1 = 0;
    let mut s2 = 0;

    // first loop
    let mut i = 0;
    while i < a {
        s1 = s1 + i;
        i += 1;
    }

    // second loop
    let mut j = 0;
    while j < b {
        s2 = s2 + (2 * j + 1);
        j += 1;
    }

    s1 + s2
}

#[no_mangle]
pub extern "C" fn main(a: i32, b: i32) -> i32 {
    let _ = two_sums(a, b);
    0
}

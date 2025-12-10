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

// -------------------------
// Logic function
// -------------------------
#[no_mangle]
pub extern "C" fn triple_nested(a: i32, b: i32, c: i32) -> i32 {
    let mut acc = 0;

    let mut i = 0;
    while i < a {
        let mut j = 0;
        while j < b {
            let mut k = 0;
            while k < c {
                acc += 1;    // linear arithmetic
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    acc
}

// -------------------------
// main calls logic
// -------------------------
#[no_mangle]
pub extern "C" fn main(a: i32, b: i32, c: i32) -> i32 {
    let r = triple_nested(a, b, c);
    0
}

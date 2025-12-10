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
pub extern "C" fn conditional_counter(a: i32) -> i32 {
    let mut state = 0;
    let mut i = 0;

    while i < a {
        if (i & 1) == 0 {
            state = state + i;     // even: add i
        } else {
            state = state - 1;     // odd: subtract 1
        }
        i += 1;
    }

    state
}

#[no_mangle]
pub extern "C" fn main(a: i32) -> i32 {
    let _ = conditional_counter(a);
    0
}

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

#[no_mangle]

pub extern "C" fn state_machine(n: i32) -> i32 {
    let mut state = 0;
    let mut i = 0;

    while i < n {
        state = 2 * state + i;
        i += 1;
    }

    state
}

#[no_mangle]
pub extern "C" fn main(n: i32) -> i32 {
    let _ = state_machine(n);
    0
}

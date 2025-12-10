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
// Counts how many steps until reaching 0
// -------------------------------------


// requires(n >= 0)
// ensures(result == n)
#[no_mangle]
pub extern "C" fn countdown(n: i32) -> i32 {
    let mut steps = 0;
    let mut x = n;

    while x > 0 {
        x -= 1;
        steps += 1;
    }

    steps
}

#[no_mangle]
pub extern "C" fn main(n: i32) -> i32 {
    let _ = countdown(n);
    0
}

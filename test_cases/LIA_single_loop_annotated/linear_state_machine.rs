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
// Linear update inside a single loop
// -------------------------------------


// requires (n >= 0)
// ensures (result == compute_linear_update(n))
#[no_mangle]
pub extern "C" fn linear_update(n: i32) -> i32 {
    let mut state = 0;
    let mut i = 0;

    while i < n {
        state = state + 3 * i + 1;
        i += 1;
    }

    state
}

#[no_mangle]
pub extern "C" fn main(n: i32) -> i32 {
    let _ = linear_update(n);
    0
}

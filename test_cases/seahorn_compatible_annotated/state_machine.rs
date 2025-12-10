#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// -------------------------------
// Required for no_std / no_main
// -------------------------------
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}



// requires(a >= 0 && b >= 0)
// ensures(result == compute_state(a, b))

pub fn state_machine(a: i32, b: i32) -> i32 {
    let mut state = 0;
    let mut i = 0;

    while i < a {
        let mut j = 0;

        while j < b {
            // linear state update rule
            state = state + 2*i - j;
            j += 1;
        }

        i += 1;
    }

    state
}

#[no_mangle]
pub extern "C" fn main(a: i32, b: i32) -> i32 {
    let _ = state_machine(a, b);
    0
}
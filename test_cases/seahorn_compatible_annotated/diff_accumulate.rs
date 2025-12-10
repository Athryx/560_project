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


// requires(n >= 0)
// ensures(result == sum_diff(n))

#[no_mangle]
pub extern "C" fn diff_accumulate(n: i32) -> i32 {
    let mut acc = 0;

    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < n {
            acc += i - j;   // linear arithmetic
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
pub extern "C" fn main(n: i32) -> i32 {
    let r = diff_accumulate(n);
    
    0
}

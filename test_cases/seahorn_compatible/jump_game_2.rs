#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// ------------------------------------------
// Required runtime stubs for no_std
// ------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// =========================================================
// Jump Game II (Greedy) — SeaHorn Compatible Version
// =========================================================

const N: usize = 5;

#[no_mangle]
pub extern "C" fn jump(xs: &[i32; N]) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut out: i32 = 0;

    // while r + 1 < len
    while r + 1 < N {
        let mut max_reach = r;
        let end = r + 1;   // iterate over [l, end]

        let mut j = l;
        while j < end {
            // compute reach = j + xs[j]
            let step = xs[j];
            let mut reach = j;

            if step > 0 {
                // manually saturating add without traits/panics
                let add = step as usize;
                if j > core::usize::MAX - add {
                    reach = core::usize::MAX;
                } else {
                    reach = j + add;
                }
            }

            if reach > max_reach {
                max_reach = reach;
            }

            j += 1;
        }

        l = end;
        r = max_reach;

        out += 1;
    }

    out
}


// =========================================================
// SeaHorn entry point: test the logic
// =========================================================

#[no_mangle]
pub extern "C" fn main(arr: [i32; N]) -> i32 {
    let result = jump(&arr);

    0
}

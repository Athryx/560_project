#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// ---- Language items & panic handler ----

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// ----------------------------
//  Struct Frame (no derives!)
// ----------------------------

struct Frame {
    target: i32,
    index: usize,
}

impl Frame {
    const fn new(t: i32, i: usize) -> Frame {
        Frame { target: t, index: i }
    }
}

const MAX_STACK: usize = 64;
const ZERO_FRAME: Frame = Frame::new(0, 0);


// ------------------------------------------------
//  Combination Sum using explicit DFS stack
// ------------------------------------------------

#[no_mangle]
pub extern "C" fn combination_sum(candidates: &[i32; 4], target: i32) -> i32 {
    let n = 4;

    let mut count: i32 = 0;

    // Explicit stack of frames (no derive, but we can repeat ZERO_FRAME)
    let mut stack: [Frame; MAX_STACK] = [ZERO_FRAME; MAX_STACK];
    let mut sp: usize = 0;

    // Push initial frame
    stack[sp] = Frame::new(target, 0);
    sp += 1;

    while sp > 0 {
        // pop
        sp -= 1;

        // COPY the frame (no borrowing!)
        let frame = Frame::new(stack[sp].target, stack[sp].index);

        if frame.target < 0 {
            continue;
        }

        if frame.target == 0 {
            count += 1;
            continue;
        }

        // try all candidates from frame.index to end
        let mut i = frame.index;
        while i < n {
            if sp < MAX_STACK {
                let new_target = frame.target - candidates[i];
                stack[sp] = Frame::new(new_target, i);  // safe
                sp += 1;
            }
            i += 1;
        }
    }

    count
}


// ------------------------------------------------
//  SeaHorn entry point
// ------------------------------------------------

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let candidates: [i32; 4] = [2, 3, 6, 7];
    let res = combination_sum(&candidates, 7);

    // Valid:
    //   [2,2,3]
    //   [7]
    // So total = 2
    assert!(res == 2);

    0
}

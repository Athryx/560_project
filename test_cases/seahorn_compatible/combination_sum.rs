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

// ---- Combination Sum logic ----

const MAX_STACK: usize = 64;

// Each DFS frame holds just the remaining target and the index in candidates.
#[derive(Copy, Clone)]
struct Frame {
    target: i32,
    index: usize,
}

// Counts the number of combinations that sum to `target`
// using unlimited reuse of candidates[i] (i.e., classic LeetCode Combination Sum).
fn combination_sum(candidates: &[i32], target: i32) -> i32 {
    let n = candidates.len();
    if n == 0 {
        return 0;
    }

    let mut count: i32 = 0;

    // Explicit fixed-size stack
    let mut stack: [Frame; MAX_STACK] = [Frame { target: 0, index: 0 }; MAX_STACK];
    let mut sp: usize = 0;

    // Push initial frame
    stack[sp] = Frame { target, index: 0 };
    sp += 1;

    while sp > 0 {
        // pop
        sp -= 1;
        let frame = stack[sp];

        if frame.target < 0 {
            // overshoot, discard
            continue;
        }

        if frame.target == 0 {
            // found a valid combination
            count += 1;
            continue;
        }

        // Try using candidates[i] for i >= frame.index
        let mut i = frame.index;
        while i < n {
            if sp < MAX_STACK {
                let new_target = frame.target - candidates[i];
                stack[sp] = Frame {
                    target: new_target,
                    index: i, // allow reuse of candidates[i]
                };
                sp += 1;
            }
            i += 1;
        }
    }

    count
}

// ---- Entry point for SeaHorn ----

#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Same test as your earlier example: [2,3,6,7], target = 7
    let candidates = [2, 3, 6, 7];
    let res = combination_sum(&candidates, 7);

    // For this instance, the valid combinations are:
    //  - [2,2,3]
    //  - [7]
    // so there should be exactly 2.
    assert!(res == 2);

    0
}

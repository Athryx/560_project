#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// ---- Required for no_std + no_main ----

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// ======================================================
//  delete_operation_for_two_strings (SeaHorn compatible)
// ======================================================

// const MAX_M: usize = 8;   // supports up to 8 chars; small for demo

#[no_mangle]
pub extern "C" fn delete_operation_for_two_strings(
    w1: &[u8],
    w2: &[u8]
) -> i32 {
    let n1 = w1.len();
    let n2 = w2.len();

    // dp[j] = LCS for prefix of w1[..i] and w2[..j]
    let mut dp: [usize; 256] = [0; 256];

    let mut i = 0;
    while i < n1 {
        let b1 = w1[i];
        let mut prev = dp[0];

        let mut j = 0;
        while j < n2 {
            let b2 = w2[j];

            // save dp[j+1] before overwriting
            let old = dp[j + 1];

            // compute dp[j+1]
            let mut val = dp[j + 1];
            if dp[j] > val {
                val = dp[j];
            }
            if b1 == b2 {
                let t = prev + 1;
                if t > val {
                    val = t;
                }
            }

            prev = old;
            dp[j + 1] = val;

            j += 1;
        }

        i += 1;
    }

    let lcs = dp[n2];
    let deletions = (n1 + n2 - 2 * lcs) as i32;
    deletions
}


// ======================================================
//  SeaHorn entry point
// ======================================================

#[no_mangle]
pub extern "C" fn main(w1: &[u8], w2: &[u8]) -> i32 {
    let result = delete_operation_for_two_strings(w1, w2);

    0
}

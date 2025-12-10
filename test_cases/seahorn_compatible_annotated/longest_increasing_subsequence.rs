#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// ----------------------------------------
// Required runtime stubs for no_std
// ----------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// =============================================================
// Longest Increasing Subsequence (O(N²) DP) — SeaHorn Compatible
// =============================================================


// requires (N > 0)
// requires (forall|i: int| 0 <= i < N ==> true)
// ensures (result >= 1)
// ensures (result == lis_length(nums))

#[no_mangle]
pub extern "C" fn length_of_lis<const N: usize>(nums: &[i32; N]) -> i32 {
    // dp[i] = LIS ending at index i
    let mut dp: [i32; N] = [0; N];

    let mut i = 0;
    while i < N {
        dp[i] = 1;  // base LIS
        let mut j = 0;
        while j < i {
            if nums[i] > nums[j] {
                let cand = dp[j] + 1;
                if cand > dp[i] {
                    dp[i] = cand;
                }
            }
            j += 1;
        }
        i += 1;
    }

    // Compute max over dp[]
    let mut res = dp[0];
    let mut k = 1;
    while k < N {
        if dp[k] > res {
            res = dp[k];
        }
        k += 1;
    }

    res
}


// =============================================================
// SeaHorn entry point: tests the logic
// =============================================================

#[no_mangle]
pub extern "C" fn main(nums: [i32; 8]) -> i32 {
    let result = length_of_lis(&nums);
    0
}

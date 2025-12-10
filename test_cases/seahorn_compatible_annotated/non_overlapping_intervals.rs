#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// ------------------------------------------
// Required runtime stubs
// ------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// ======================================================
// SeaHorn-Compatible erase_overlap_intervals
// ======================================================


// requires (intervals.len() == N)
// requires (forall|i: int|
//     0 <= i < N ==> intervals[i].len() == 2)
// requires (forall|i: int|
//     0 <= i < N ==> intervals[i][0] <= intervals[i][1]) 
// ensures (result >= 0)
// ensures (result == min_removals_to_non_overlapping(old(intervals)))

const N: usize = 4; // number of intervals

#[no_mangle]
pub extern "C" fn erase_overlap_intervals(
    intervals: &mut [[i32; 2]; N]
) -> i32 {

    // ------------------------------------------
    // Bubble-sort intervals by start time
    // ------------------------------------------
    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j + 1 < N - i {
            if intervals[j][0] > intervals[j + 1][0] {
                // swap
                let t0 = intervals[j][0];
                let t1 = intervals[j][1];

                intervals[j][0]     = intervals[j + 1][0];
                intervals[j][1]     = intervals[j + 1][1];

                intervals[j + 1][0] = t0;
                intervals[j + 1][1] = t1;
            }
            j += 1;
        }
        i += 1;
    }

    // ------------------------------------------
    // Compute number of intervals to remove
    // ------------------------------------------
    let mut cnt_del: i32 = 0;
    let mut prev: usize = 0;

    i = 1;
    while i < N {
        let cur_start = intervals[i][0];
        let cur_end   = intervals[i][1];
        let prev_start = intervals[prev][0];
        let prev_end   = intervals[prev][1];

        // overlap condition
        if cur_start < prev_end && cur_start >= prev_start {
            cnt_del += 1;

            // keep the interval with smaller end
            if prev_end > cur_end {
                prev = i;
            }
        } else {
            prev = i;
        }

        i += 1;
    }

    cnt_del
}


// ======================================================
// SeaHorn entry point (fixed test case)
// ======================================================

#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Original example:
    // [1,2], [2,3], [3,4], [1,3]
    let mut intervals: [[i32; 2]; N] = [
        [1,2],
        [2,3],
        [3,4],
        [1,3],
    ];

    let result = erase_overlap_intervals(&mut intervals);

    // For this set: remove 1 interval
    assert!(result == 1);

    0
}

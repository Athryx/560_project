#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// ---------------------------------------------
// Required for no_std
// ---------------------------------------------
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// ============================================================
// Maximal Rectangle (fixed 4×5 version)
// ============================================================


// requires(matrix.len() == ROWS)
// requires(forall|i: int| 0 <= i < ROWS ==> matrix[i].len() == COLS)
// requires(forall|i: int, j: int|
//     0 <= i < ROWS && 0 <= j < COLS ==> matrix[i][j] == 0 || matrix[i][j] == 1)
// ensures(result >= 0)]
// ensures(result == maximal_area(matrix))


const ROWS: usize = 4;
const COLS: usize = 5;

#[no_mangle]
pub extern "C" fn maximal_rectangle(
    matrix: &[[u8; COLS]; ROWS]  // use u8 instead of char
) -> i32 {
    // heights array per row
    let mut heights: [[i32; COLS]; ROWS] = [[0; COLS]; ROWS];

    // Build heights[][]
    let mut i = 0;
    while i < ROWS {
        let mut j = 0;
        while j < COLS {
            if matrix[i][j] == 1 {
                if i == 0 {
                    heights[i][j] = 1;
                } else {
                    heights[i][j] = heights[i - 1][j] + 1;
                }
            } else {
                heights[i][j] = 0;
            }
            j += 1;
        }
        i += 1;
    }

    let mut max_area: i32 = 0;

    // Process each row using histogram stack
    i = 0;
    while i < ROWS {
        // stack stores column indices
        let mut stack: [usize; COLS + 1] = [0; COLS + 1];
        let mut sp: usize = 0;

        // Loop through columns + 1 (extra zero height)
        let mut j = 0;
        while j <= COLS {
            let cur_h = if j == COLS { 0 } else { heights[i][j] };

            // while (stack not empty) and (height at top > cur_h)
            while sp > 0 {
                let top_j = stack[sp - 1];   // peek
                let top_h = heights[i][top_j];

                if top_h <= cur_h {
                    break;
                }

                // pop
                sp -= 1;
                let height = top_h;

                // compute width
                let width = if sp == 0 {
                    j as i32
                } else {
                    (j - stack[sp - 1] - 1) as i32
                };

                let area = height * width;
                if area > max_area {
                    max_area = area;
                }
            }

            // push j
            stack[sp] = j;
            sp += 1;

            j += 1;
        }

        i += 1;
    }

    max_area
}


// ==============================================
// SeaHorn entry point
// ==============================================

#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Convert '1','0' chars to u8 {1,0}
    let matrix: [[u8; COLS]; ROWS] = [
        [1,0,1,0,0],
        [1,0,1,1,1],
        [1,1,1,1,1],
        [0,0,0,1,0],
    ];

    let result = maximal_rectangle(&matrix);

    // For this matrix, maximal rectangle area = 6
    assert!(result == 6);

    0
}

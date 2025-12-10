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


// ========================================================
// Game of Life for fixed 4x3 board (SeaHorn compatible)
// ========================================================

const ROWS: usize = 4;
const COLS: usize = 3;



//requires (board.len() == ROWS && next.len() == ROWS)
//requires (forall|i: int| 0 <= i < ROWS ==> board[i].len() == COLS && next[i].len() == COLS)
//requires (forall|i: int, j: int|
//    0 <= i < ROWS && 0 <= j < COLS ==> board[i][j] == 0 || board[i][j] == 1)
//ensures (forall|i: int, j: int|
//    0 <= i < ROWS && 0 <= j < COLS ==> next[i][j] == life_rule(board, i, j))

// next[i][j] = f(board[i][j], live_neighbors)
#[no_mangle]
pub extern "C" fn game_of_life(
    board: &[[i32; COLS]; ROWS],
    next: &mut [[i32; COLS]; ROWS],
) {
    let mut i = 0;
    while i < ROWS {
        let mut j = 0;
        while j < COLS {

            // Count live neighbors
            let mut cnt = 0;

            // Convert i,j to signed for neighbor iteration
            let ii = i as isize;
            let jj = j as isize;

            let mut di = -1;
            while di <= 1 {
                let mut dj = -1;
                while dj <= 1 {

                    if !(di == 0 && dj == 0) {
                        let ni = ii + di;
                        let nj = jj + dj;

                        if ni >= 0 && ni < ROWS as isize &&
                           nj >= 0 && nj < COLS as isize {

                            let bi = ni as usize;
                            let bj = nj as usize;

                            cnt += board[bi][bj] & 1;
                        }
                    }

                    dj += 1;
                }
                di += 1;
            }

            let cur = board[i][j] & 1;
            let mut cell = 0;

            // Birth or survival rules (no match expression)
            if cur == 1 {
                if cnt == 2 || cnt == 3 {
                    cell = 1;
                }
            } else {
                if cnt == 3 {
                    cell = 1;
                }
            }

            next[i][j] = cell;
            j += 1;
        }
        i += 1;
    }
}


// ========================================================
// SeaHorn entry point (tests the logic)
// ========================================================

#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Original board:
    // [0,1,0]
    // [0,0,1]
    // [1,1,1]
    // [0,0,0]

    let board: [[i32; COLS]; ROWS] = [
        [0, 1, 0],
        [0, 0, 1],
        [1, 1, 1],
        [0, 0, 0],
    ];

    let mut next: [[i32; COLS]; ROWS] = [
        [0; COLS],
        [0; COLS],
        [0; COLS],
        [0; COLS],
    ];

    game_of_life(&board, &mut next);

    // We assert one known property of the next board:
    // next[1][0] should be 1 in the standard rules for this input.
    // (This is just a sanity check so SeaHorn has a postcondition.)
    assert!(next[1][0] == 1);

    0
}

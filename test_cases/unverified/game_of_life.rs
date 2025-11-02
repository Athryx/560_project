use vstd::prelude::*;

fn main () {}

verus! {

fn game_of_life(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = board.len();
    if rows == 0 { return vec![]; }
    let cols = board[0].len();
    if cols == 0 { return vec![vec![]]; }

    let mut next = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(0);
        }
        next.push(row);
    }

    // let mut next = vec![vec![0; cols]; rows];

    for i in 0..rows {
        let mut next_row = Vec::with_capacity(cols); // build a whole row
        
        for _ in 0..cols {
            next_row.push(0);
        }

        for j in 0..cols {
            // count live neighbors from original board
            let mut cnt = 0;
            let (ii, jj) = (i as isize, j as isize);
            for di in -1..2 {
                for dj in -1..2 {
                    if di != 0 || dj != 0 {
                        let ni = ii + di;
                        let nj = jj + dj;
                        if 0 <= ni && ni < rows as isize && 0 <= nj && nj < cols as isize {
                            cnt += board[ni as usize][nj as usize] & 1;
                        }
                    }
                }
            }

            let cur = board[i][j] & 1;
            next_row[j] = match (cur, cnt) {
                (1, 2) | (1, 3) => 1, // stays alive
                (0, 3) => 1,          // reproduction
                _ => 0,               // dies or stays dead
            };
        }
        next[i] = next_row; // assign the completed row
    }

    next
}

}// verus!
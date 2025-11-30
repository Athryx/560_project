#![no_std]

const MAX_STACK: usize = 64;      // max number of frames on our explicit stack
const MAX_PATH_LEN: usize = 8;    // max length of any combination (depth)

// One DFS frame: similar to (path, target, index) in your original code,
// but path is a fixed-size array plus a separate length.
#[derive(Copy, Clone)]
struct Frame {
    path: [i32; MAX_PATH_LEN],
    path_len: usize,
    target: i32,
    index: usize,
}

// Returns: number of distinct combinations that sum to `target`
// using unlimited reuse of candidates (like the original LeetCode problem).
fn combination_sum(candidates: &[i32], target: i32) -> i32 {
    let n = candidates.len();
    if n == 0 {
        return 0;
    }

    let mut count: i32 = 0;

    // Explicit stack of frames instead of Vec<(Vec<i32>, i32, usize)>
    let mut stack: [Frame; MAX_STACK] = [Frame {
        path: [0; MAX_PATH_LEN],
        path_len: 0,
        target: 0,
        index: 0,
    }; MAX_STACK];

    // stack pointer (next free slot)
    let mut sp: usize = 0;

    // push the initial frame: empty path, full target, start at index 0
    stack[sp] = Frame {
        path: [0; MAX_PATH_LEN],
        path_len: 0,
        target,
        index: 0,
    };
    sp += 1;

    // Iterative DFS using our explicit stack
    while sp > 0 {
        sp -= 1;
        let frame = stack[sp];

        if frame.target < 0 {
            // overshot the target, discard
            continue;
        } else if frame.target == 0 {
            // found a valid combination
            count += 1;
            continue;
        }

        // Try extending the current path with candidates[i], i from index..n
        let mut i = frame.index;
        while i < n {
            // Only push if we have stack capacity and path capacity
            if frame.path_len < MAX_PATH_LEN && sp < MAX_STACK {
                let mut new_path = frame.path;
                new_path[frame.path_len] = candidates[i]; // safe: < MAX_PATH_LEN

                stack[sp] = Frame {
                    path: new_path,
                    path_len: frame.path_len + 1,
                    target: frame.target - candidates[i],
                    index: i, // reuse i to allow repeated use of candidates[i]
                };
                sp += 1;
            }
            i += 1;
        }
    }

    count
}

// A small concrete main so SeaHorn has an entry point.
#[no_mangle]
pub extern "C" fn main() -> i32 {
    let candidates = [2, 3, 6, 7];
    combination_sum(&candidates, 7)
}

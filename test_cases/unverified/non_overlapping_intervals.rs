use vstd::prelude::*;

fn main () {}

verus! {

fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut cnt_del : i32 = 0;
    let mut prev    : usize = 0;

    let mut intervals_mut = intervals;

    // Replace built-in sort() with bubble sort implementation
    let n = intervals_mut.len();
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j + 1 < n - i {
            if intervals_mut[j][0] > intervals_mut[j + 1][0] {
                let temp = intervals_mut[j].clone();
                intervals_mut[j] = intervals_mut[j + 1].clone();
                intervals_mut[j + 1] = temp;
            }
            j += 1;
        }
        i += 1;
    }

    for i in 1..intervals_mut.len() {
        if intervals_mut[i][0] < intervals_mut[prev][1] && intervals_mut[i][0] >= intervals_mut[prev][0] {
            cnt_del = cnt_del + 1;
            if intervals_mut[prev][1] > intervals_mut[i][1] {
                prev = i;
            }
        } else {
            prev = i;
        }
    }
    
    return cnt_del;
}

} // verus! 
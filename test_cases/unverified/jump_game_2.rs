use vstd::prelude::*;

fn main () {}

verus! {

fn jump(xs: &Vec<i32>) -> (result: i32)
{
    let (mut l, mut r) = (0usize, 0usize);
    let mut out = 0i32;
    let len: usize = xs.len();

    while r + 1 < len {
        let mut max_reach = r;
        let end = r + 1; // iterate over [l, end)
        for j in l..end {
            let reach = j.saturating_add(xs[j] as usize);
            if reach > max_reach {
                max_reach = reach;
            }
        }
        l = end;
        r = max_reach;
        out += 1;
    }
    out
}

} // verus!
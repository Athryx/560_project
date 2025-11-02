use vstd::prelude::*;

fn main () {}

verus! {

fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut res = 1;
    let mut dp = Vec::with_capacity(n);
    // let mut i=0;
    // let mut j=0;

    for i in 0..n{
        dp.push(1);
        for j in 0..i+1{
            if nums[i] > nums[j]{
                dp[i] = dp[i].max(dp[j]+1);
            }
        }
        res = res.max(dp[i]);
    }
    res
}

} // verus!
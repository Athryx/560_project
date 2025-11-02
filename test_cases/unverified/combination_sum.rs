use std::collections::btree_map::IntoValues;

use vstd::prelude::*;

fn main () {}

verus! {

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut stack: Vec<(Vec<i32>, i32, usize)> = Vec::new();

    stack.push((vec![], target, 0));

    loop 
    {
        let Some((mut path, target, index)) = stack.pop() else { break };

        if target < 0 {
            continue;
        } else if target == 0 {
            result.push(path);
        } else {
            for i in index..candidates.len() {
                path.push(candidates[i]);
                stack.push((path.clone(), target - candidates[i], i));
                path.pop();
            }
        }
    }

    result
}

} // verus!
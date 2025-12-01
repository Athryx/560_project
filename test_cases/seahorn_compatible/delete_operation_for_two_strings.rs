fn main () {
    let word1 = String::from("sea");
    let word2 = String::from("eat");
    let result = delete_operation_for_two_strings(word1, word2);
    println!("Minimum deletions: {}", result);
}

    
fn delete_operation_for_two_strings(word1: String, word2: String) -> i32 
    // requires
    //     word1.len() <= 500,
    //     word2.len() <= 500,
    // ensures
    //     result >= 0,
    //     forall|i: int| (0 <= i && i < word1.len()) ==> (0 <= i && i < word2.len()) ==> (word1.as_bytes()[i] == word2.as_bytes()[i]),
{
    const MAX_W2: usize = 4096; // upper bound for word2 length
    let s2 = word2.as_bytes();
    
    // Fixed-size DP buffer on the stack (no Vec, no enumerate)
    let mut dp: [usize; MAX_W2 + 1] = [0; MAX_W2 + 1];
    let m = s2.len(); // only use dp[..=m]

    for &b1 in word1.as_bytes() 
    // invariant 
    //     word1.len() <= 500,
    //     word2.len() <= 500,
    //     m == word2.len(),
    //     forall|j: int| (0 <= j && j <= m) ==> dp[j] <= j,
    //     forall|j: int| (0 <= j && j < dp.len()) ==> dp[j] as int >= 0,
    //     forall|i: int| (0 <= i && i < word1.len()) ==> (0 <= i && i < word2.len()) ==> (word1.as_bytes()[i] == word2.as_bytes()[i]),
    // decreases word1.len() - ( { let mut count = 0; for &b in word1.as_bytes() { count += 1; if b == b1 { break; } } count } ),
    {
        let mut prev = dp[0];
        for j in 0..m 
        // invariant
        //     word1.len() <= 500,
        //     word2.len() <= 500,
        //     m == word2.len(),
        //     0 <= j && j <= m,
        //     forall|jj: int| (0 <= jj && jj < j) ==> dp[jj] <= jj,
        //     forall|jj: int| (0 <= jj && jj < dp.len()) ==> dp[jj] as int >= 0,
        //     forall|i: int| (0 <= i && i < word1.len()) ==> (0 <= i && i < word2.len()) ==> (word1.as_bytes()[i] == word2.as_bytes()[i]),
        // decreases m - j,
        {
            let b2 = s2[j];
            let val = dp[j + 1].max(if b1 == b2 { prev + 1 } else { dp[j] });
            prev = dp[j + 1];
            dp[j + 1] = val;
        }
    }

    let lcs = dp[m];
    (word1.len() + word2.len() - 2 * lcs) as i32
}
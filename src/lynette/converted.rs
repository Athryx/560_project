use vstd::prelude::*;
fn inc(n: u8) -> u8 {
    if n == 5 {
        n + 3
    } else {
        n + 1
    }
}
fn inc_assert_pre_check(n: u8) -> bool {
    n < 255
}
fn inc_assert_post_check(n: u8, r: u8) {
    assert!(
        (!(n == 8) || (r == n + 1))
            && ({
                let condition = |n| n > 5;
                let mut result = true;
                {
                    for arg0 in (i64::MIN)..=(i64::MAX) {
                        {
                            if condition(arg0) == false {
                                result = false;
                                break;
                            }
                        }
                    }
                };
                result
            })
    )
}
fn inc_assert_wrapper(arg0: u8) {
    if inc_assert_pre_check(arg0) {
        let result = inc(arg0);
        inc_assert_post_check(arg0, result);
    };
}

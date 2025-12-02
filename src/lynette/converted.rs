use vstd::prelude::*;
use vstd::relations::sorted_by;
fn sorted(arr: Seq<i32>) -> bool {
    {
        let condition = |i1: int, i2: int| !(((0 <= i1) < i2) < arr.len()) || (arr[i1] <= arr[i2]);
        let mut result = true;
        {
            for arg1 in (i64::MIN)..=(i64::MAX) {
                {
                    for arg0 in (i64::MIN)..=(i64::MAX) {
                        {
                            if condition(arg0, arg1) == false {
                                result = false;
                                break;
                            }
                        }
                    }
                }
            }
        };
        result
    }
}
fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    let mut i = 0;
    while i < n {
        assert!(
            ((((0 <= i) <= n) && (arr.len() == n))
                && ({
                    let condition = |i1: int, i2: int| {
                        !(((((n - i) - 1) < i1) < i2) < n) || (arr[i1] <= arr[i2])
                    };
                    let mut result = true;
                    {
                        for arg1 in (i64::MIN)..=(i64::MAX) {
                            {
                                for arg0 in (i64::MIN)..=(i64::MAX) {
                                    {
                                        if condition(arg0, arg1) == false {
                                            result = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    };
                    result
                }))
                && ({
                    let condition = |i1: int, i2: int| {
                        !((((0 <= i1) < (n - i)) <= i2) < n) || (arr[i1] <= arr[i2])
                    };
                    let mut result = true;
                    {
                        for arg1 in (i64::MIN)..=(i64::MAX) {
                            {
                                for arg0 in (i64::MIN)..=(i64::MAX) {
                                    {
                                        if condition(arg0, arg1) == false {
                                            result = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    };
                    result
                })
        );
        let mut swapped = false;
        let mut j = 0;
        while j < (n - i - 1) {
            assert!(
                ((((((0 <= j) <= ((n - i) - 1)) && (arr.len() == n))
                    && ({
                        let condition =
                            |elem: int| !((0 <= elem) < j) || (arr[elem] <= arr[j as int]);
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
                    }))
                    && ({
                        let condition = |i1: int, i2: int| {
                            !(((((n - i) - 1) < i1) < i2) < n) || (arr[i1] <= arr[i2])
                        };
                        let mut result = true;
                        {
                            for arg1 in (i64::MIN)..=(i64::MAX) {
                                {
                                    for arg0 in (i64::MIN)..=(i64::MAX) {
                                        {
                                            if condition(arg0, arg1) == false {
                                                result = false;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        result
                    }))
                    && ({
                        let condition = |i1: int, i2: int| {
                            !((((0 <= i1) < (n - i)) <= i2) < n) || (arr[i1] <= arr[i2])
                        };
                        let mut result = true;
                        {
                            for arg1 in (i64::MIN)..=(i64::MAX) {
                                {
                                    for arg0 in (i64::MIN)..=(i64::MAX) {
                                        {
                                            if condition(arg0, arg1) == false {
                                                result = false;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        result
                    }))
                    && (!(!swapped)
                        || ({
                            let condition =
                                |i1: int, i2: int| !(((0 <= i1) < i2) <= j) || (arr[i1] <= arr[i2]);
                            let mut result = true;
                            {
                                for arg1 in (i64::MIN)..=(i64::MAX) {
                                    {
                                        for arg0 in (i64::MIN)..=(i64::MAX) {
                                            {
                                                if condition(arg0, arg1) == false {
                                                    result = false;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            result
                        }))
            );
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
                swapped = true;
            }
            j += 1;
            assert!(
                ((((((0 <= j) <= ((n - i) - 1)) && (arr.len() == n))
                    && ({
                        let condition =
                            |elem: int| !((0 <= elem) < j) || (arr[elem] <= arr[j as int]);
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
                    }))
                    && ({
                        let condition = |i1: int, i2: int| {
                            !(((((n - i) - 1) < i1) < i2) < n) || (arr[i1] <= arr[i2])
                        };
                        let mut result = true;
                        {
                            for arg1 in (i64::MIN)..=(i64::MAX) {
                                {
                                    for arg0 in (i64::MIN)..=(i64::MAX) {
                                        {
                                            if condition(arg0, arg1) == false {
                                                result = false;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        result
                    }))
                    && ({
                        let condition = |i1: int, i2: int| {
                            !((((0 <= i1) < (n - i)) <= i2) < n) || (arr[i1] <= arr[i2])
                        };
                        let mut result = true;
                        {
                            for arg1 in (i64::MIN)..=(i64::MAX) {
                                {
                                    for arg0 in (i64::MIN)..=(i64::MAX) {
                                        {
                                            if condition(arg0, arg1) == false {
                                                result = false;
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        result
                    }))
                    && (!(!swapped)
                        || ({
                            let condition =
                                |i1: int, i2: int| !(((0 <= i1) < i2) <= j) || (arr[i1] <= arr[i2]);
                            let mut result = true;
                            {
                                for arg1 in (i64::MIN)..=(i64::MAX) {
                                    {
                                        for arg0 in (i64::MIN)..=(i64::MAX) {
                                            {
                                                if condition(arg0, arg1) == false {
                                                    result = false;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            };
                            result
                        }))
            );
        }
        if !swapped {
            i = n;
            break;
        }
        i += 1;
        assert!(
            ((((0 <= i) <= n) && (arr.len() == n))
                && ({
                    let condition = |i1: int, i2: int| {
                        !(((((n - i) - 1) < i1) < i2) < n) || (arr[i1] <= arr[i2])
                    };
                    let mut result = true;
                    {
                        for arg1 in (i64::MIN)..=(i64::MAX) {
                            {
                                for arg0 in (i64::MIN)..=(i64::MAX) {
                                    {
                                        if condition(arg0, arg1) == false {
                                            result = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    };
                    result
                }))
                && ({
                    let condition = |i1: int, i2: int| {
                        !((((0 <= i1) < (n - i)) <= i2) < n) || (arr[i1] <= arr[i2])
                    };
                    let mut result = true;
                    {
                        for arg1 in (i64::MIN)..=(i64::MAX) {
                            {
                                for arg0 in (i64::MIN)..=(i64::MAX) {
                                    {
                                        if condition(arg0, arg1) == false {
                                            result = false;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    };
                    result
                })
        );
    }
}
fn bubble_sort_assert_pre_check(arr: &mut Vec<i32>) -> bool {
    true
}
fn bubble_sort_assert_post_check(arr: &mut Vec<i32>, result: ()) {
    assert!(sorted(arr.view()))
}
fn bubble_sort_assert_wrapper(arg0: &mut Vec<i32>) {
    if bubble_sort_assert_pre_check(arg0) {
        let result = bubble_sort(arg0);
        bubble_sort_assert_post_check(arg0, result);
    };
}

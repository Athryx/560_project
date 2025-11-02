fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    let mut swapped;

    for i in 0..n {
        swapped = false;
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        // If no swaps happened, the array is already sorted
        if !swapped {
            break;
        }
    }
}

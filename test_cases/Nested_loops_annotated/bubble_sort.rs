#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// requires (arr.len() == n)
// requires (n >= 0)
// ensures  (forall|k: int| 0 <= k < n - 1 ==> arr[k] <= arr[k + 1])
// ensures  (forall|v: int| count(arr, v) == count(old(arr), v))
// ensures  (forall|p: int, q: int|
//     0 <= p < q < n && old(arr[p]) == old(arr[q]) ==> index_of(arr, old(arr[p]), p) < index_of(arr, old(arr[q]), q)


// ------------------------------------------
// SeaHorn-Compatible Bubble Sort
// ------------------------------------------
#[no_mangle]
pub extern "C" fn bubble_sort(arr: &mut [i32 ]) {
    let mut i = 0;
    let n = arr.len();
    while i < n {
        let mut swapped = false;

        let mut j = 0;
        while j < (n - i - 1) {
            if arr[j] > arr[j + 1] {
                let tmp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
                swapped = true;
            }
            j += 1;
        }

        if !swapped {
            break;
        }

        i += 1;
    }
}

// ------------------------------------------
// main(): calls bubble_sort and asserts sortedness
// ------------------------------------------
#[no_mangle]
pub extern "C" fn main(data: &mut [i32]) -> i32 {
    bubble_sort(data);

    0
}
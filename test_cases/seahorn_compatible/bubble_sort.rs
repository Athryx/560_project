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


// ------------------------------------------
// SeaHorn-Compatible Bubble Sort
// ------------------------------------------
#[no_mangle]
pub extern "C" fn bubble_sort(arr: &mut [i32; 7]) {
    let n = 7;

    let mut i = 0;
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
pub extern "C" fn main() -> i32 {
    let mut data: [i32; 7] = [64, 34, 25, 12, 22, 11, 90];

    bubble_sort(&mut data);

    // Optional property checks for verification
    let mut k = 0;
    while k < 6 {
        assert!(data[k] <= data[k + 1]);   // array is sorted
        k += 1;
    }

    0
}

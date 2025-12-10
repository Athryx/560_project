#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

use core::panic::PanicInfo;

// -------------------------------
// Required for no_std / no_main
// -------------------------------
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// ===============================================
// Recursive Modular Exponentiation (SeaHorn-compatible)
// ===============================================
#[no_mangle]
pub extern "C" fn pow(base: u64, exponent: u64, modulo: u64) -> u64 {
    if exponent == 0 {
        return 1;
    } else if exponent == 1 {
        return base % modulo;
    }

    let result = pow(base, exponent >> 1, modulo);

    if (exponent & 1) != 0 {
        // (result * result * base) % modulo
        return ((result * result) % modulo * base) % modulo;
    } else {
        // (result * result) % modulo
        return (result * result) % modulo;
    }
}


// ===============================================
// SeaHorn Entry Point
// ===============================================

#[no_mangle]
pub extern "C" fn main(base: u64, exponent: u64, modulo: u64) -> u64 {
    let r = pow(base, exponent, modulo);
    r as u64
}

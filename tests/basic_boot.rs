#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kilonova::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kilonova::{println, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("One line!");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("Many lines!");
    }
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kilonova::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kilonova::{println, serial_println, init};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
	init();
    test_main();
    loop {}
}

#[test_case]
fn breakpoint_interrupt() {
	x86_64::instructions::interrupts::int3();
}

#![no_main]
#![no_std]
#![feature(custom_test_frameworks, abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod gdt;
pub mod int;
pub mod serial;
pub mod test;
pub mod vga;
pub mod volatile;
pub mod mem;

#[cfg(test)]
use test::{exit_qemu, QemuExitCode};

use gdt::init_gdt;
use int::{halt, init_idt, init_pic};
use mem::init_opt;

pub fn init() {
    init_idt();
    init_gdt();
    init_pic();
	unsafe { init_opt(); }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello there!");
    test_main();
    halt();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt();
}

#[cfg(test)]
#[panic_handler]
fn test_panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    halt();
}

#[macro_export]
macro_rules! entry {
    ($func:ident) => {
        #[no_mangle]
        pub extern "C" fn _start(boot_info: &'static bootloader::BootInfo) -> ! {
            $func(boot_info);
        }
    };
}

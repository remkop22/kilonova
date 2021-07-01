#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kilonova::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kilonova::{entry, init, println};
use kilonova::mem::translate_addr;
use bootloader::BootInfo;
use x86_64::VirtAddr;

entry!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    init();
    println!("Hello there");
	let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    kilonova::int::halt();
}

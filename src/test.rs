use crate::{serial_println, serial_print};

#[cfg(test)]
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T where T: Fn() {
	fn run(&self) {
		serial_print!("{} => ", core::any::type_name::<T>());
		self();
		serial_println!("[OK]")
	}
}

pub enum QemuExitCode {
    Succes = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Succes);
}

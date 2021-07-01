use crate::gdt::DOUBLE_FAULT_IST_INDEX;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

mod handlers;

pub fn init_idt() {
    IDT.load();
}

pub fn init_pic() {
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
	Keyboard
}

pub fn halt() -> ! {
	loop {
		x86_64::instructions::hlt();
	}
}

impl Into<u8> for InterruptIndex {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<usize> for InterruptIndex {
    fn into(self) -> usize {
        self as usize
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
 
		idt.breakpoint.set_handler_fn(handlers::breakpoint_hanlder);
		idt.page_fault.set_handler_fn(handlers::page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(handlers::double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        
		idt[InterruptIndex::Timer.into()].set_handler_fn(handlers::timer_handler);
		idt[InterruptIndex::Keyboard.into()].set_handler_fn(handlers::keyboard_handler);
  
		idt
    };
}

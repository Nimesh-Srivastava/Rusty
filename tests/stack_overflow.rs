#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use rusty::{exit_qemu, QemuExitCode, serial_print, serial_println};

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);

    loop {}
}

lazy_static! {
    static ref TEST_IDT:InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.double_fault.set_handler_fn(test_double_fault_handler).set_stack_index(rusty::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    rusty::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty::test_panic_handler(info)
}

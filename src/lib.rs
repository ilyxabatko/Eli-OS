#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::testable::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod macros;
pub mod qemu_utils;
pub mod serial_port;
pub mod testable;
pub mod vga_buffer;

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::testable::test_panic_handler;

    test_panic_handler(info)
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

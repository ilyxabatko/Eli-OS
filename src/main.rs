#![no_main]
#![no_std]
// replacing the default test framework that requires standard library (which is turned off ofc)
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// rename the generated "main" function by the "custom_test_frameworks"
// then call it from the "_start" fn (entrypoint)
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod macros;
mod qemu_utils;
mod serial_port;
mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, {}", "Eli!");

    #[cfg(test)] // this function is only generated in "test" condition
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", &info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::qemu_utils::{QemuExitCode, exit_qemu};

    serial_println!("[FAILED]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::qemu_utils::{QemuExitCode, exit_qemu};

    serial_println!("Running {} tests:", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial_assertion... ");
    assert_eq!(2, 1 + 1 - 1);
    serial_println!("[SUCCESS]");
}

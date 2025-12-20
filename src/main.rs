#![no_std]
#![no_main]
// replacing the default test framework that requires standard library (which is turned off ofc)
#![feature(custom_test_frameworks)]
#![test_runner(eli_os::testable::test_runner)]
// rename the generated "main" function by the "custom_test_frameworks"
// then call it from the "_start" fn (entrypoint)
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use eli_os::println;

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
    eli_os::testable::test_panic_handler(info)
}

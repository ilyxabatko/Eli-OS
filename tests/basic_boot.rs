#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(eli_os::testable::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use eli_os::halt_loop;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    halt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eli_os::testable::test_panic_handler(info)
}

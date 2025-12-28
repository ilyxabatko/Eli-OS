#![no_std]
#![no_main]
// replacing the default test framework that requires standard library (which is turned off ofc)
#![feature(custom_test_frameworks)]
#![test_runner(eli_os::testable::test_runner)]
// rename the generated "main" function by the "custom_test_frameworks"
// then call it from the "_start" fn (entrypoint)
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use eli_os::{
    allocator, halt_loop,
    memory::{self, BootInfoFrameAllocator},
    println,
};
use x86_64::VirtAddr;

extern crate alloc;

// Defines a type-checked entrypoint for our kernel
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, {}", "Eli!");
    eli_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Error initializing a heap");

    #[cfg(test)] // this function is only generated in "test" condition
    test_main();

    halt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", &info);
    halt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    eli_os::testable::test_panic_handler(info)
}

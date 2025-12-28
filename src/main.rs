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
    halt_loop,
    memory::{self, BootInfoFrameAllocator},
    println,
};
use x86_64::{VirtAddr, structures::paging::Page};

// Defines a type-checked entrypoint for our kernel
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, {}", "Eli!");
    eli_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page at the bery beginning of the virtual address space
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // map an unused page
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

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

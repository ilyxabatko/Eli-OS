use x86_64::instructions::port::Port;

const QEMU_DEBUG_EXIT_PORT: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(QEMU_DEBUG_EXIT_PORT);
        port.write(exit_code as u32);
    }
}

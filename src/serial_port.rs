use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

const FIRST_SERIAL_INTERFACE_PORT: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL_FIRST: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(FIRST_SERIAL_INTERFACE_PORT) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    x86_64::instructions::interrupts::without_interrupts(|| {
        SERIAL_FIRST
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

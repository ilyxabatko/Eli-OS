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
    SERIAL_FIRST
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host through the serial interface
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial_port::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a new line
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

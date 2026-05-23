// src/serial.rs

// serial port: https://wiki.osdev.org/Serial_Ports
// serial port is used to communicate with the outside hardware
// :)

use core::fmt; // format arguments
use lazy_static::lazy_static; // lazy static
use spin::Mutex; // mutex
use uart_16550::{backend::PioBackend, Config, Uart16550}; // uart 16550

lazy_static! { // lazy static initial
    pub static ref SERIAL1: Mutex<Uart16550<PioBackend>> = Mutex::new(unsafe { // create serial port, unsafe because it's accessing hardware directly
        let mut serial_port =
            Uart16550::new_port(0x3f8).expect("Failed to create serial port"); // create a serial port at addr 0x3f8
        serial_port
            .init(Config::default())
            .expect("Failed to initialize serial port"); // initialize the serial port
        serial_port
    });
}

#[doc(hidden)] // hidden from documentation
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    struct SerialWriter<'a> {
        port: &'a mut Uart16550<PioBackend>,
    }

    impl fmt::Write for SerialWriter<'_> { // implement write for serial writer
        fn write_str(&mut self, s: &str) -> fmt::Result { // write string to serial port
            self.port.send_bytes_exact(s.as_bytes());
            Ok(())
        }
    }

    let mut serial_port = SERIAL1.lock(); // lock the serial port
    SerialWriter {// create serial writer
        port: &mut serial_port,
    }
    .write_fmt(args)
    .expect("Printing to serial failed"); // print to serial
}

#[macro_export] // export macro

/// print to serial port
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export] // export macro

/// print to serial port with newline
macro_rules! serial_println {
    () => {
        $crate::serial::_print(format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!("{}\n", format_args!($($arg)*)))
    };
}

////////////////////////////////////////////
// end of file                            //
////////////////////////////////////////////
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::{backend::PioBackend, Config, Uart16550};

lazy_static! {
    pub static ref SERIAL1: Mutex<Uart16550<PioBackend>> = Mutex::new(unsafe {
        let mut serial_port =
            Uart16550::new_port(0x3f8).expect("Failed to create serial port");
        serial_port
            .init(Config::default())
            .expect("Failed to initialize serial port");
        serial_port
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    struct SerialWriter<'a> {
        port: &'a mut Uart16550<PioBackend>,
    }

    impl fmt::Write for SerialWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.port.send_bytes_exact(s.as_bytes());
            Ok(())
        }
    }

    let mut serial_port = SERIAL1.lock();
    SerialWriter {
        port: &mut serial_port,
    }
    .write_fmt(args)
    .expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::serial::_print(format_args!("\n"))
    };
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!("{}\n", format_args!($($arg)*)))
    };
}
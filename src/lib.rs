// src/lib.rs

// compiler directives
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


// modules
pub mod serial;
pub mod vga_buffer;

// panic handler
use core::panic::PanicInfo;

// qemu exit codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode { // qemu exit codes
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) { // exit qemu
    use x86_64::instructions::port::Port;

    unsafe { // unsafe because of port I/O
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// testable trait
pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T // implement testable for all functions
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// test runner
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// test panic handler
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
// test entry point
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { 
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}


////////////////////////////////////////////
// end of file                            //
////////////////////////////////////////////
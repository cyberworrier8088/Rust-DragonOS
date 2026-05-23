// tests/basic_boot.rs

// basic boot test for Rust_DragonOS.

// compiler directives
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Rust_DragonOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

// use for panic handler and println
use core::panic::PanicInfo;
use Rust_DragonOS::println;

// entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}


// panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { 
    Rust_DragonOS::test_panic_handler(info)
}

// test case
#[test_case]
fn test_println() { // test println
    println!("test_println output");
}


////////////////////////////////////////////
// end of file                            //
////////////////////////////////////////////
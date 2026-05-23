// This OS made using Rust programming language
// credits: https://github.com/phil-opp/blog_os
// https://os.phil-opp.com/
// This made that inspired by those sources
// :}
// Enjoy!
// comiler directives
#![no_std] // remove std lib
#![no_main] // remove main function entry point
#![feature(custom_test_frameworks)] // enable custom test framework
#![test_runner(Rust_DragonOS::test_runner)] // set test runner
#![reexport_test_harness_main = "test_main"] // reexport test main

// use needed crates and func in anthor files
use core::panic::PanicInfo; // panic handler
use Rust_DragonOS::println; // print func 
use Rust_DragonOS::vga_buffer::WRITER; // vga buffer


// entry point
#[unsafe(no_mangle)] // disable name mangling
pub extern "C" fn _start() -> ! { // c entry point

    // Terminall printing banner
    {
    let mut writer = WRITER.lock();
    writer.clear_screen();

    writer.write_centered(5,  "                 / \\  //\\\\");
    writer.write_centered(6,  "        |\\___/|      \\\\//  \\\\");
    writer.write_centered(7,  "        /O  O  \\__   //    \\\\");
    writer.write_centered(8,  "       /     /  \\/_/      //");
    writer.write_centered(9,  "       \\_^_\\'/   \\/_   _//");
    writer.write_centered(10, "       //_^_/     \\/_ //");
    writer.write_centered(11, "    ( //) |        \\///");
    writer.write_centered(12, "  ( / /) _|_ /   )  //");
    writer.write_centered(13, " ( // /) '/,_ _ _/  ( ;");
    writer.write_centered(15, "        === DragonOS ===");
    writer.write_centered(17, "      DragonOS v0.1.0");
}
    #[cfg(test)]
    test_main();

    loop {}
}

// panic handler
#[cfg(not(test))] // cfg = configuration, not test
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// test panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Rust_DragonOS::test_panic_handler(info)
}

// test case
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1); // assert that 1 equals 1
}

////////////////////////////////////////////
// end of file                            //
////////////////////////////////////////////
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_os::println;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main(); // #![reexport_test_harness_main = "test_main"]

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    my_os::test_panic_handler(info)
}

// fn test_runner(tests: &[&dyn Fn()]) {
//     unimplemented!();
// }
//
// #[test_case]
// fn trivial_assertion() {
//     // serial_print!("trivial assertion... ");
//     assert_eq!(1, 1);
//     // serial_println!("[ok]");
// }

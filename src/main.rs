#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rock_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to Rock OS{}", "!");
    // panic!("Some panic message");

    rock_os::init();

    #[cfg(test)]
    test_main();
    println!("Enter the Password {}", "");
    println!("User Name:{}", "admin");
    println!("Password: {}", "");

    rock_os::hlt_loop();

}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rock_os::hlt_loop();

}

#[cfg(test)]
#[panic_handler]
fn test_runner(tests: &[&dyn Fn()]) {
    rock_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    rusty::init();
    
    #[cfg(test)]
    test_main();

    println!("It did not crash");
    rusty::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rusty::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty::test_panic_handler(info);
    loop{}
}

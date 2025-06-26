#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world{}", "!");

    rusty::init();
    
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

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

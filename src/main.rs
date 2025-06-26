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
    //use rusty::memory::active_level_4_table;
    use x86_64::VirtAddr;
    use rusty::memory::translate_addr;

    println!("Hello world{}", "!");
    rusty::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [0xb8000, 0x201008, 0x0100_0020_1a10, boot_info.physical_memory_offset,];
    
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }


    //let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    //
    //for (i, entry) in l4_table.iter().enumerate() {
    //    use x86_64::structures::paging::PageTable;
    //
    //    if !entry.is_unused() {
    //        println!("L4 entry {}: {:?}", i, entry);
    //
    //        let phys = entry.frame().unwrap().start_address();
    //        let virt = phys.as_u64() + boot_info.phys_memory_offset;
    //        let ptr = VirtAddr::new(virt).as_mut_ptr();
    //        let l3_table: &&PageTable = unsafe { &*ptr };
    //
    //        for (i, entry) in l3_table.iter().enumerate() {
    //            if !entry.is_unused() {
    //                println!("L3 Entry {}: {:?}", i, entry);
    //            }
    //        }
    //    }
    //}
    

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

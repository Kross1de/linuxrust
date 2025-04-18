#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(linuxre::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use linuxre::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    linuxre::init();
    x86_64::instructions::interrupts::int3();
    
    #[cfg(test)]
    test_main();

    println!("it's not crashed! =)");
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
    linuxre::test_panic_handler(info)
}


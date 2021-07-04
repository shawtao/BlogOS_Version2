#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(OSTrain::test_runner)]
#![reexport_test_harness_main = "test_main"]

use OSTrain::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    OSTrain::test_panic_handler(_info);
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    //vga_buffer::print_something();
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}


#[cfg(test)]
pub mod test {

    use crate::*;
    use OSTrain::{serial_println,serial_print};

    #[test_case]
    fn trivial_assertion() {
        serial_print!("trivial assertion... ");
        assert_eq!(0, 1);
        serial_println!("[ok]");
    }

}
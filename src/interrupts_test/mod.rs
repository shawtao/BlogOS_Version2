pub mod idt;

use lazy_static::lazy_static;
use crate::println;

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();
        idt.set_handler(3, breakpoint_handler);
        idt
    };
}

extern "C" fn breakpoint_handler() -> ! {
    println!("EXCEPTION: BreakPoint!");
    loop {}
}

pub fn init() {
     IDT.load();
}
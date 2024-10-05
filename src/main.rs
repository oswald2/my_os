#![no_std]
#![no_main]
use core::{arch::global_asm, panic::PanicInfo};

use serial::Serial;

#[macro_use]
mod print;
mod serial;

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

const STACK_SIZE: usize = 4 * 1024;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        kprintln!("Panicked at: {}\r\n{}", location, info.message());
    } else {
        kprintln!("Panicked at unknown location:\r\n{}", info.message());
    }

    loop {}
}

#[no_mangle]
fn main() {
    let serial = Serial::new();

    kprintln!("It works!");

    kprintln!("Serial: {:?}", serial);

    loop {}
}

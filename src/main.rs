#![no_std]
#![no_main]
use core::{arch::global_asm, panic::PanicInfo};

#[no_mangle]
#[link_section = ".stack"]
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

const STACK_SIZE: usize = 4 * 1024;

global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline]
fn serial_putchar(c: char) {
    let uart_addr = 0x0900_0000 as *mut u8;

    unsafe {
        *uart_addr = c as u8;
    }
}

fn serial_puts(s: &str) {
    for c in s.chars() {
        serial_putchar(c);
    }
}

#[no_mangle]
fn main() {
    serial_puts("Hello World!");
    loop {}
}

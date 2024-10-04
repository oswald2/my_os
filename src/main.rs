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

#[no_mangle]
fn main() {
    let uart_addr = 0x0900_0000 as *mut u8;

    unsafe {
        uart_addr.write_volatile('A' as u8);
    }

    loop {}
}

use core::fmt::Write;

#[derive(Debug)]
pub struct Serial {
    uart_addr: *mut u8,
}

impl Serial {
    pub fn new() -> Serial {
        Serial {
            uart_addr: 0x0900_0000 as *mut u8,
        }
    }

    #[inline]
    pub fn putchar(&mut self, c: char) {
        unsafe {
            self.uart_addr.write_volatile(c as u8);
        }
    }

    pub fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putchar(c);
        }
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.puts(s);
        Ok(())
    }
}
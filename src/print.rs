#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut serial = crate::serial::Serial::new();
            let _ = serial.write_fmt(format_args!($($arg)*));
        }
    };
}


#[macro_export]
macro_rules! kprintln {
    () => {
        $crate::kprint!("\n\r");
    };
    ($($arg:tt)*) => {
        $crate::kprint!("{}{}", format_args!($($arg)*), "\n\r");
    }
}
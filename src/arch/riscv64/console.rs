use core::fmt::{self, Write};

use super::sbi::console_putchar;

pub fn put_fmt(args: fmt::Arguments) {
    Console.write_fmt(args).unwrap();
}

struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod syscall;
mod syscall_numbers;
mod toaru;
mod file;
mod allocator;

use crate::toaru::{write, exit};
use crate::allocator::init_allocator;
use core::panic::PanicInfo;
use core::fmt::{self};
use alloc::boxed::Box;
use alloc::vec;

struct Stdout;
impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(1, s.as_bytes());
        Ok(())
    }
}

macro_rules! print {
    ($($arg:tt)*) => {{
        let _ = core::fmt::write(&mut Stdout, format_args!($($arg)*));
    }};
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

const STDOUT: usize = 1;

#[unsafe(no_mangle)]
unsafe fn _start() {
    unsafe {
        init_allocator();
        println!("Welcome to Rust on ToaruOS!");

        let number = 42;
        print!("Number is: {}\n", number);

        if let Some(mut f) = crate::file::File::open("/etc/motd", 0, 0) {
            let mut buffer = vec![0u8; 256];
            let n = f.read(&mut buffer);
            write(STDOUT, &buffer[..n]);
            f.close();
        }

        let a = Box::new(1234usize);
        let b = Box::new(5678usize);
        let c = Box::new(0xDEADBEEFusize);

        println!("Testing multiple Box allocs:");
        println!("a = {}", *a);
        println!("b = {}", *b);
        println!("c = {:#X}", *c);

        exit(0);
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        println!("Panic at {}:{}: {}", loc.file(), loc.line(), info);
    } else {
        println!("Panic: {}", info);
    }
    loop {}
}

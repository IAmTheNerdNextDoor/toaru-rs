#![no_std]
#![no_main]

use core::panic::PanicInfo;

const SYS_WRITE: usize = 4;
const SYS_EXT: usize = 0;
const STDOUT: usize = 1;
static mut BUF: [u8; 20] = [0u8; 20];

#[unsafe(no_mangle)]
fn _start() {
    write(STDOUT, b"Welcome to Rust on ToaruOS!\n");
    let number = 42;
    write(STDOUT, b"Number is: ");
    print_number(number);
    write(STDOUT, b"\n");
    exit(0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


fn print_number(mut n: usize) {
    let mut i = 20;

    if n == 0 {
        write(STDOUT, b"0");
        return;
    }

    unsafe {
        while n > 0 && i > 0 {
            i -= 1;
            BUF[i] = b'0' + (n % 10) as u8;
            n /= 10;
        }
    
        write(STDOUT, &BUF[i..]);
    }
}

fn write(fd: usize, buf: &[u8]) -> usize {
    unsafe {
        syscall(SYS_WRITE, fd, buf.as_ptr() as usize, buf.len()) 
    }
}

fn exit(code: usize) -> ! {
    unsafe {
        syscall(SYS_EXT, code, 0, 0);
    }
    loop {}
}

#[inline(always)]
unsafe fn syscall(num: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;
    unsafe { 
        core::arch::asm!(
            "syscall",
            in("rax") num,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rax") ret,
        );
        ret
   }
}


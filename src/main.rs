#![no_std]
#![no_main]

use core::panic::PanicInfo;

const SYS_WRITE: usize = 4;
const STDOUT: usize = 1;

static HELLO: &[u8] = b"If you are seeing this, Rust is working on ToaruOS.\n";

#[unsafe(no_mangle)]
pub unsafe fn _start() {
    unsafe { 
        syscall(
            SYS_WRITE,
            STDOUT,
            HELLO.as_ptr() as usize,
            HELLO.len(),
        );
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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


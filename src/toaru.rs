extern crate alloc;

use crate::syscall::syscall;
use crate::syscall_numbers::*;

pub fn write(fd: usize, buf: &[u8]) -> usize {
    unsafe { syscall(SYS_WRITE, fd, buf.as_ptr() as usize, buf.len()) }
}

pub fn read(fd: usize, buf: &mut [u8]) -> usize {
    unsafe { syscall(SYS_READ, fd, buf.as_mut_ptr() as usize, buf.len()) }
}

pub fn open(path: &str, flags: usize, mode: usize) -> isize {
    let bytes = path.as_bytes();

    if bytes.len() >= 255 {
        return -1;
    }

    let mut nulled = [0u8; 256];
    for (i, &b) in bytes.iter().enumerate() {
        nulled[i] = b;
    }
    nulled[bytes.len()] = 0;

    unsafe {
        syscall(SYS_OPEN, nulled.as_ptr() as usize, flags, mode) as isize
    }
}

pub fn close(fd: usize) -> usize {
    unsafe { syscall(SYS_CLOSE, fd, 0, 0) }
}

pub fn exit(code: usize) -> ! {
    unsafe {
        syscall(SYS_EXT, code, 0, 0);
    }
    loop {}
}

pub fn sbrk(increment: usize) -> usize {
    unsafe { syscall(SYS_SBRK, increment, 0, 0) }
}

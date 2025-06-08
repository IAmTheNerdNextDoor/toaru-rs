#![allow(dead_code)]

extern crate alloc;

use crate::toaru::{read, write, close};
use core::ops::Drop;

pub struct File {
    fd: usize,
}

impl File {

    pub fn open(path: &str, flags: usize, mode: usize) -> Option<Self> {
        let fd = { crate::toaru::open(path, flags, mode) };
        if fd >= 0 { Some(File { fd: fd as usize }) } else { None }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        read(self.fd, buf)
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        write(self.fd, buf)
    }

    pub fn close(self) {
        let _ = close(self.fd);
    }

    pub fn seek(&mut self, offset: usize, whence: usize) -> Option<usize> {
        let res = unsafe { crate::syscall::syscall(
            super::syscall_numbers::SYS_SEEK,
            self.fd, offset, whence
        ) };
        if res != usize::MAX { Some(res) } else { None }
    }

    pub fn stat(&self, buf: &mut [u8]) -> Option<usize> {
        let res = unsafe { crate::syscall::syscall(
            super::syscall_numbers::SYS_STAT,
            self.fd, buf.as_mut_ptr() as usize, buf.len()
        ) };
        if (res as isize) >= 0 { Some(res) } else { None }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}

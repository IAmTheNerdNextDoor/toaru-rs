#[inline(always)]
pub unsafe fn syscall(
    num: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
) -> usize { unsafe {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") ret,
    );
    ret
}}

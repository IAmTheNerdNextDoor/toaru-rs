use crate::toaru::sbrk;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::hint::spin_loop;
use core::ptr::null_mut;

const HEAP_SIZE: usize = 1024 * 1024;

pub struct Bump {
    start: usize,
    end: usize,
    current: AtomicUsize,
    lock: AtomicBool,
}

unsafe impl Sync for Bump {}

unsafe impl GlobalAlloc for Bump {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        while self.lock.swap(true, Ordering::Acquire) {
            spin_loop();
        }
        let mut cur = self.current.load(Ordering::Relaxed);
        cur = (cur + layout.align() - 1) & !(layout.align() - 1);
        let ptr = if cur + layout.size() > self.end {
            null_mut()
        } else {
            self.current.store(cur + layout.size(), Ordering::Relaxed);
            cur as *mut u8
        };

        self.lock.store(false, Ordering::Release);
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        /* I'll work on this eventually */
    }
}

#[global_allocator]
static ALLOCATOR: Bump = Bump {
    start: 0,
    end: 0,
    current: AtomicUsize::new(0),
    lock: AtomicBool::new(false),
};

#[alloc_error_handler]
fn oom(layout: Layout) -> ! {
    panic!("OOM: {:?}", layout);
}

pub unsafe fn init_allocator() { unsafe {
    let base = sbrk(0);
    let _ = sbrk(HEAP_SIZE);
    let ptr           = &ALLOCATOR as *const _ as *mut Bump;
    (*ptr).start      = base;
    (*ptr).end        = base + HEAP_SIZE;
    (*ptr).current.store(base, Ordering::Relaxed);
}}

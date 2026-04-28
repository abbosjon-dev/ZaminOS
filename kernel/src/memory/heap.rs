//! Kernel heap allokatori.
//!
//! `linked_list_allocator` cratesi ustida sodda global allokator.
//! Heap kernel obrazidan yuqori joyda, stack ostida joylashadi.
//!
//! Layout (QEMU virt RAM 0x40000000 dan 512 MiB):
//!     [kernel ELF .text/.rodata/.data/.bss] [stack 64 KiB] [HEAP 4 MiB]
//!
//! `__kernel_end` linker tomonidan beriladi. Stack `__kernel_end + 64 KiB` da,
//! heap esa stack ortidan boshlanadi.

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use linked_list_allocator::Heap;

const STACK_SIZE: usize = 64 * 1024;
pub const HEAP_SIZE: usize = 4 * 1024 * 1024; // 4 MiB

extern "C" {
    static __kernel_end: u8;
}

struct LockedHeap {
    inner: UnsafeCell<Heap>,
}

unsafe impl Sync for LockedHeap {}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: bir-yadroli, prerivaniyalar yo'q (Faza 4 da spinlock).
        let heap = &mut *self.inner.get();
        heap.allocate_first_fit(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |p| p.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let heap = &mut *self.inner.get();
        heap.deallocate(core::ptr::NonNull::new_unchecked(ptr), layout);
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap {
    inner: UnsafeCell::new(Heap::empty()),
};

/// Heap'ni ishga tushir. Bir marta `kernel_main` boshida chaqiriladi.
pub unsafe fn init() {
    let kernel_end = core::ptr::addr_of!(__kernel_end) as usize;
    let heap_start = kernel_end + STACK_SIZE;
    let heap = &mut *ALLOCATOR.inner.get();
    heap.init(heap_start as *mut u8, HEAP_SIZE);
}

/// Diagnostika uchun.
pub fn stats() -> (usize, usize, usize) {
    unsafe {
        let heap = &*ALLOCATOR.inner.get();
        (heap.size(), heap.used(), heap.free())
    }
}

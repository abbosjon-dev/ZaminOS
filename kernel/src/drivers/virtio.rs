//! virtio-drivers cratesining `Hal` traitini ZaminOS uchun amalga oshirish.
//!
//! Bizda identity-mapped MMU bor — virtual va fizik manzillar teng.
//! DMA bufferlari oddiy heap'dan ajratiladi (4 KiB alignment bilan).

use alloc::alloc::{alloc_zeroed, dealloc, Layout};
use core::ptr::NonNull;
use virtio_drivers::{BufferDirection, Hal, PhysAddr};

const PAGE_SIZE: usize = 4096;

pub struct ZaminHal;

unsafe impl Hal for ZaminHal {
    fn dma_alloc(pages: usize, _direction: BufferDirection) -> (PhysAddr, NonNull<u8>) {
        let layout = Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap();
        let ptr = unsafe { alloc_zeroed(layout) };
        if ptr.is_null() {
            panic!("virtio dma_alloc: heap tugadi ({} sahifa)", pages);
        }
        (ptr as PhysAddr, unsafe { NonNull::new_unchecked(ptr) })
    }

    unsafe fn dma_dealloc(_paddr: PhysAddr, vaddr: NonNull<u8>, pages: usize) -> i32 {
        let layout = Layout::from_size_align(pages * PAGE_SIZE, PAGE_SIZE).unwrap();
        dealloc(vaddr.as_ptr(), layout);
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: PhysAddr, _size: usize) -> NonNull<u8> {
        NonNull::new_unchecked(paddr as *mut u8)
    }

    unsafe fn share(buffer: NonNull<[u8]>, _direction: BufferDirection) -> PhysAddr {
        // Identity-mapped: virtual = physical.
        buffer.as_ptr() as *mut u8 as PhysAddr
    }

    unsafe fn unshare(_paddr: PhysAddr, _buffer: NonNull<[u8]>, _direction: BufferDirection) {
        // No-op identity mapping uchun.
    }
}

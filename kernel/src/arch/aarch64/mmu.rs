//! Minimal aarch64 MMU sozlash.
//!
//! Strategiya: bir L1 jadval, 1 GiB blok deskriptorlari.
//! Adres maydoni 39 bitli (T0SZ=25). Birinchi 4 GiB identity-mapped:
//!   - 0x0000_0000 .. 0x4000_0000  : Device-nGnRnE (MMIO: GIC, UART, virtio)
//!   - 0x4000_0000 .. 0x8000_0000  : Normal cacheable (asosiy RAM)
//!   - 0x8000_0000 .. 0xC000_0000  : Normal cacheable (qo'shimcha RAM)
//!   - 0xC000_0000 .. 0x1_0000_0000: Normal cacheable
//!
//! Bu bizga UART, GIC, virtio, va 192 GiB gacha RAM bilan ishlash imkonini beradi
//! — Faza 5 (framebuffer) va undan keyin ham yetarli.

use aarch64_cpu::asm::barrier;
use aarch64_cpu::registers::*;
use core::arch::asm;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

/// MAIR indekslari.
const MAIR_DEVICE: u64 = 0;
const MAIR_NORMAL: u64 = 1;

/// L1 blok deskriptori bayroqlari.
const VALID: u64 = 1 << 0;
const BLOCK: u64 = 0 << 1; // L0/L1 da: bit[1]=0 => block, =1 => table
const AF: u64 = 1 << 10; // Access flag
const SH_INNER: u64 = 0b11 << 8; // Inner shareable

#[inline]
const fn attr_idx(idx: u64) -> u64 {
    idx << 2
}

#[repr(C, align(4096))]
struct PageTable {
    entries: [u64; 512],
}

static mut L1_TABLE: PageTable = PageTable { entries: [0; 512] };

/// MMU ni yoqish. `kernel_main` boshida bir marta chaqiriladi.
pub unsafe fn init() {
    // 1. L1 jadvalni to'ldir: 4 ta 1 GiB blok.
    //    L1[0] => MMIO (Device), L1[1..4] => RAM (Normal cacheable).
    let table = &mut *core::ptr::addr_of_mut!(L1_TABLE);

    table.entries[0] = build_block(0x0000_0000, MAIR_DEVICE);
    table.entries[1] = build_block(0x4000_0000, MAIR_NORMAL);
    table.entries[2] = build_block(0x8000_0000, MAIR_NORMAL);
    table.entries[3] = build_block(0xC000_0000, MAIR_NORMAL);
    // Qolgan entrylar 0 (invalid) — tegilmaydi.

    // 2. MAIR_EL1: xotira atributlari jadvali.
    //    Index 0: Device-nGnRnE (0x00)
    //    Index 1: Normal Inner/Outer Write-Back, Read/Write-Allocate (0xff)
    MAIR_EL1.write(
        MAIR_EL1::Attr0_Device::nonGathering_nonReordering_noEarlyWriteAck
            + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
            + MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc,
    );

    // 3. TCR_EL1: 39-bit VA, 4 KiB sahifalar, IPS=40-bit.
    TCR_EL1.write(
        TCR_EL1::TBI0::Used
            + TCR_EL1::IPS::Bits_40
            + TCR_EL1::TG0::KiB_4
            + TCR_EL1::SH0::Inner
            + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
            + TCR_EL1::EPD0::EnableTTBR0Walks
            + TCR_EL1::T0SZ.val(25)
            + TCR_EL1::EPD1::DisableTTBR1Walks
            + TCR_EL1::T1SZ.val(25),
    );

    // 4. TTBR0_EL1 -> L1 jadval bazasi.
    let l1_pa = core::ptr::addr_of!(L1_TABLE) as u64;
    TTBR0_EL1.set_baddr(l1_pa);

    // 5. TLB ni tozala, barrier.
    asm!("tlbi vmalle1is", options(nostack));
    barrier::dsb(barrier::ISH);
    barrier::isb(barrier::SY);

    // 6. SCTLR_EL1: M (MMU), C (data cache), I (instr cache) bitlarini yoq.
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

    barrier::isb(barrier::SY);
}

/// 1 GiB blok deskriptorini yasash.
const fn build_block(pa: u64, mair_index: u64) -> u64 {
    pa | attr_idx(mair_index) | SH_INNER | AF | BLOCK | VALID
}

/// Joriy execution levelni qaytaradi (debug uchun).
pub fn current_el() -> u64 {
    CurrentEL.read(CurrentEL::EL)
}

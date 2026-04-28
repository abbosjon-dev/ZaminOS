//! GIC v2 (Generic Interrupt Controller) — QEMU `virt` machine.
//!
//! QEMU virt memory map:
//!   0x0800_0000  GIC Distributor (GICD)
//!   0x0801_0000  GIC CPU Interface (GICC)
//!
//! Bu drayver minimal: faqat bir nechta umumiy SPI / PPI uchun.
//! Faza 7 da GICv3 (Pi 5) qo'shiladi.

use core::ptr::{read_volatile, write_volatile};

const GICD_BASE: usize = 0x0800_0000;
const GICC_BASE: usize = 0x0801_0000;

// Distributor registrlari (ofsetlari).
const GICD_CTLR: usize = 0x000;
const GICD_TYPER: usize = 0x004;
const GICD_ISENABLER: usize = 0x100; // 32 IRQ / register
const GICD_IPRIORITYR: usize = 0x400; // 4 IRQ / register
const GICD_ITARGETSR: usize = 0x800; // 4 IRQ / register
const GICD_ICFGR: usize = 0xC00; // 16 IRQ / register

// CPU interface registrlari.
const GICC_CTLR: usize = 0x00;
const GICC_PMR: usize = 0x04;
const GICC_IAR: usize = 0x0C;
const GICC_EOIR: usize = 0x10;

#[inline]
unsafe fn mmio_read(addr: usize) -> u32 {
    read_volatile(addr as *const u32)
}

#[inline]
unsafe fn mmio_write(addr: usize, val: u32) {
    write_volatile(addr as *mut u32, val);
}

pub unsafe fn init() {
    // Distributor: hozircha hammasini o'chir.
    mmio_write(GICD_BASE + GICD_CTLR, 0);

    // Qancha IRQ liniya bor (faqat ma'lumot uchun).
    let _typer = mmio_read(GICD_BASE + GICD_TYPER);

    // CPU interface: prioritet maskasini eng pastga (barcha prioritetlar o'tadi).
    mmio_write(GICC_BASE + GICC_PMR, 0xff);

    // Distributor va CPU interface ni yoq.
    mmio_write(GICD_BASE + GICD_CTLR, 1);
    mmio_write(GICC_BASE + GICC_CTLR, 1);
}

/// IRQ ni yoqish (SPI yoki PPI).
pub unsafe fn enable_irq(irq: u32) {
    let reg = (irq / 32) as usize;
    let bit = irq % 32;
    mmio_write(GICD_BASE + GICD_ISENABLER + reg * 4, 1 << bit);

    // Prioritetni o'rta darajaga (0xa0).
    let preg = (irq / 4) as usize;
    let pshift = (irq % 4) * 8;
    let cur = mmio_read(GICD_BASE + GICD_IPRIORITYR + preg * 4);
    let new = (cur & !(0xff << pshift)) | (0xa0 << pshift);
    mmio_write(GICD_BASE + GICD_IPRIORITYR + preg * 4, new);

    // Target CPU 0 (SPI uchun; PPI har CPU ga avtomatik bog'lanadi).
    if irq >= 32 {
        let treg = (irq / 4) as usize;
        let tshift = (irq % 4) * 8;
        let cur = mmio_read(GICD_BASE + GICD_ITARGETSR + treg * 4);
        let new = (cur & !(0xff << tshift)) | (0x01 << tshift);
        mmio_write(GICD_BASE + GICD_ITARGETSR + treg * 4, new);
    }
}

/// Joriy IRQ raqamini olib, handler chaqirib, end-of-interrupt yuboradi.
pub fn handle_irq() {
    let iar = unsafe { mmio_read(GICC_BASE + GICC_IAR) };
    let irq = iar & 0x3ff;

    if irq == 1023 {
        // Spurious — e'tibor bermaymiz.
        return;
    }

    match irq {
        30 => crate::drivers::timer::on_tick(),
        _ => crate::println!("[gic] kutilmagan IRQ #{}", irq),
    }

    unsafe { mmio_write(GICC_BASE + GICC_EOIR, iar) };
}

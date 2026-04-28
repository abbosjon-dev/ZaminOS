//! ARMv8 Generic Timer — periodik tick.
//!
//! Virtual timer (PPI #27 + 16 = IRQ 27)... aslida QEMU virt'da:
//!   - Secure Phys Timer:    IRQ 29 (PPI 13)
//!   - Non-secure Phys Timer: IRQ 30 (PPI 14)
//!   - Virtual Timer:         IRQ 27 (PPI 11)
//!   - Hyp Phys Timer:        IRQ 26 (PPI 10)
//!
//! Biz EL1 da turibmiz, NS Phys Timer (IRQ 30) ni ishlatamiz.

use aarch64_cpu::registers::*;
use core::sync::atomic::{AtomicU64, Ordering};
use tock_registers::interfaces::{Readable, Writeable};

pub const TIMER_IRQ: u32 = 30;

static TICK_COUNT: AtomicU64 = AtomicU64::new(0);
static FREQ_HZ: AtomicU64 = AtomicU64::new(0);

/// Timerni `interval_ms` millisekund interval bilan ishga tushirish.
pub unsafe fn init(interval_ms: u64) {
    let freq = CNTFRQ_EL0.get();
    FREQ_HZ.store(freq, Ordering::Relaxed);

    let ticks = (freq * interval_ms) / 1000;

    // Birinchi tick uchun comparator.
    CNTP_TVAL_EL0.set(ticks);

    // CTL: ENABLE=1, IMASK=0
    CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);
}

/// Bir tickdan keyin chaqiriladi (GIC handler dan).
pub fn on_tick() {
    let count = TICK_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    // Keyingi intervalni qayta o'rnat (1 sekund).
    let freq = FREQ_HZ.load(Ordering::Relaxed);
    CNTP_TVAL_EL0.set(freq);

    crate::println!("[timer] tick #{} (uptime ~ {} s)", count, count);
}

#[allow(dead_code)]
pub fn ticks() -> u64 {
    TICK_COUNT.load(Ordering::Relaxed)
}

//! AArch64 exception handlerlari.
//!
//! Vector table `exceptions.S` da, bu yerda Rust handlerlari.

use aarch64_cpu::registers::*;
use core::arch::global_asm;
use tock_registers::interfaces::Writeable;

global_asm!(include_str!("exceptions.S"));

extern "C" {
    static exception_vector_table: u8;
}

/// VBAR_EL1 ni vector tableга yo'naltir va prerivaniyalarni yoqishga tayyorla.
pub unsafe fn init() {
    let vbar = core::ptr::addr_of!(exception_vector_table) as u64;
    VBAR_EL1.set(vbar);
}

/// IRQ larni global yoqish (DAIF.I = 0).
pub unsafe fn enable_irqs() {
    core::arch::asm!("msr daifclr, #2", options(nostack));
}

/// IRQ larni global o'chirish (DAIF.I = 1).
#[allow(dead_code)]
pub unsafe fn disable_irqs() {
    core::arch::asm!("msr daifset, #2", options(nostack));
}

#[no_mangle]
extern "C" fn handle_sync(_regs: *mut u8, esr: u64, elr: u64) {
    crate::println!(
        "[exc] SYNC: ESR=0x{:016x} ELR=0x{:016x} EC={}",
        esr,
        elr,
        (esr >> 26) & 0x3f
    );
    panic!("Sinxron istisno");
}

#[no_mangle]
extern "C" fn handle_irq() {
    crate::drivers::gicv2::handle_irq();
}

#[no_mangle]
extern "C" fn handle_fiq() {
    crate::println!("[exc] FIQ — kutilmagan");
}

#[no_mangle]
extern "C" fn handle_serror() {
    crate::println!("[exc] SError");
    panic!("SError");
}

#[no_mangle]
extern "C" fn handle_unknown() {
    crate::println!("[exc] Unknown vector");
    panic!("Unknown exception");
}

//! ZaminOS kernel — aarch64, Rust, mobile-first konvergent OS.
//!
//! Faza 0–1: yuklanadi, UART orqali salom xabari chiqaradi, cheksiz tsiklga o'tadi.

#![no_std]
#![no_main]

use core::arch::global_asm;

mod console;
mod drivers;
mod panic;

// Boot stub (boot.S) ni kernelga qo'shamiz.
global_asm!(include_str!("boot.S"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!();
    println!("==============================================");
    println!(" ZaminOS v0.1.0  —  aarch64 Rust kernel");
    println!(" Salom, dunyo! Yadro muvaffaqiyatli yuklandi.");
    println!("==============================================");
    println!();
    println!("[boot] CPU: cortex-a72 (QEMU virt)");
    println!("[boot] UART: PL011 @ 0x09000000");
    println!("[boot] Faza 0 + 1 OK. Faza 2 (MMU) keyingi qadam.");
    println!();
    println!("[idle] Yadro idle tsiklga o'tdi (wfe).");

    loop {
        unsafe { core::arch::asm!("wfe") };
    }
}

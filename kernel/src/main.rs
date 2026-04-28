//! ZaminOS kernel — aarch64, Rust, mobile-first konvergent OS.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::arch::global_asm;

mod arch;
mod console;
mod drivers;
mod memory;
mod panic;
mod task;

// Boot stub (boot.S) ni kernelga qo'shamiz.
global_asm!(include_str!("boot.S"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!();
    println!("==============================================");
    println!(" ZaminOS v0.2.0  —  aarch64 Rust kernel");
    println!(" Salom, dunyo! Yadro muvaffaqiyatli yuklandi.");
    println!("==============================================");
    println!();

    let el = arch::aarch64::mmu::current_el();
    println!("[boot] Joriy Exception Level: EL{}", el);
    println!("[boot] CPU: cortex-a72 (QEMU virt)");
    println!("[boot] UART: PL011 @ 0x09000000");

    // --- Faza 2: MMU + heap ---
    print!("[mmu] Sozlash... ");
    unsafe {
        arch::aarch64::mmu::init();
    }
    println!("OK (39-bit VA, 1 GiB bloklar, identity map)");

    print!("[heap] Sozlash... ");
    unsafe {
        memory::heap::init();
    }
    let (size, used, free) = memory::heap::stats();
    println!(
        "OK (size={} KiB, used={} B, free={} KiB)",
        size / 1024,
        used,
        free / 1024
    );

    // --- alloc demo ---
    println!();
    println!("[demo] Heap allokatorni sinab ko'ramiz:");

    let boxed: Box<u64> = Box::new(0xDEAD_BEEF_CAFE_BABE);
    println!("  Box<u64> = 0x{:016X}", *boxed);

    let mut nums: Vec<u32> = Vec::new();
    for i in 1..=10 {
        nums.push(i * i);
    }
    println!("  Vec<u32> kvadratlar (1..=10): {:?}", nums.as_slice());

    let greeting = String::from("Salom, ZaminOS heap'idan!");
    println!("  String: \"{}\"", greeting);

    let (_, used, free) = memory::heap::stats();
    println!(
        "[heap] allokatsiyalardan keyin: used={} B, free={} KiB",
        used,
        free / 1024
    );

    drop(nums);
    drop(greeting);
    drop(boxed);
    let (_, used, free) = memory::heap::stats();
    println!(
        "[heap] dealloc'lardan keyin:    used={} B, free={} KiB",
        used,
        free / 1024
    );

    // --- Faza 3: exceptions + GIC + timer ---
    println!();
    print!("[exc] Vector table o'rnatish... ");
    unsafe {
        arch::aarch64::exceptions::init();
    }
    println!("OK");

    print!("[gic] Sozlash... ");
    unsafe {
        drivers::gicv2::init();
        drivers::gicv2::enable_irq(drivers::timer::TIMER_IRQ);
    }
    println!("OK (GICv2 @ 0x08000000)");

    print!("[timer] Generic timer ishga tushirish (1 Hz)... ");
    unsafe {
        drivers::timer::init(1000);
    }
    println!("OK");

    // --- Faza 4: kooperativ scheduler demo (IRQ lardan oldin) ---
    println!();
    println!("[sched] Faza 4 demo: 3 ta task aylanma navbatda (round-robin):");
    task::spawn("alpha", |i| {
        println!("  [alpha] iter {}", i);
        i < 3
    });
    task::spawn("beta", |i| {
        println!("  [beta ] iter {}", i);
        i < 3
    });
    task::spawn("gamma", |i| {
        println!("  [gamma] iter {}", i);
        i < 3
    });
    task::run();

    // --- IRQ larni yoqib, idle (wfi) tsikliga o'tamiz ---
    println!();
    print!("[exc] IRQ larni globally yoqish... ");
    unsafe {
        arch::aarch64::exceptions::enable_irqs();
    }
    println!("OK");

    println!();
    println!("[boot] Faza 0-4 OK. Timer tickni kuzating:");
    println!();

    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

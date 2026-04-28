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
mod graphics;
mod memory;
mod panic;
mod task;

use graphics::framebuffer::{Color, Framebuffer};

global_asm!(include_str!("boot.S"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!();
    println!("==============================================");
    println!(" ZaminOS v0.5.0  —  aarch64 Rust kernel");
    println!(" Salom, dunyo! Yadro muvaffaqiyatli yuklandi.");
    println!("==============================================");
    println!();

    let el = arch::aarch64::mmu::current_el();
    println!("[boot] Joriy Exception Level: EL{}", el);
    println!("[boot] CPU: cortex-a72 (QEMU virt)");
    println!("[boot] UART: PL011 @ 0x09000000");

    print!("[mmu] Sozlash... ");
    unsafe {
        arch::aarch64::mmu::init();
    }
    println!("OK");

    print!("[heap] Sozlash... ");
    unsafe {
        memory::heap::init();
    }
    let (size, _, free) = memory::heap::stats();
    println!("OK (size={} KiB, free={} KiB)", size / 1024, free / 1024);

    println!();
    println!("[demo] Heap allokatorni sinab ko'ramiz:");
    let boxed: Box<u64> = Box::new(0xDEADBEEFCAFEBABE);
    println!("  Box<u64> = 0x{:016X}", *boxed);
    let nums: Vec<u32> = (1..=10u32).map(|i| i * i).collect();
    println!("  Vec<u32> kvadratlar: {:?}", nums.as_slice());
    let greeting = String::from("Salom, ZaminOS heap'idan!");
    println!("  String: \"{}\"", greeting);

    println!();
    print!("[exc] Vector table... ");
    unsafe {
        arch::aarch64::exceptions::init();
    }
    println!("OK");

    print!("[gic] Sozlash... ");
    unsafe {
        drivers::gicv2::init();
        drivers::gicv2::enable_irq(drivers::timer::TIMER_IRQ);
    }
    println!("OK");

    print!("[timer] Generic timer 1 Hz... ");
    unsafe {
        drivers::timer::init(1000);
    }
    println!("OK");

    println!();
    println!("[sched] Faza 4 demo:");
    task::spawn("alpha", |i| {
        println!("  [alpha] iter {}", i);
        i < 2
    });
    task::spawn("beta", |i| {
        println!("  [beta ] iter {}", i);
        i < 2
    });
    task::spawn("gamma", |i| {
        println!("  [gamma] iter {}", i);
        i < 2
    });
    task::run();

    // --- Faza 5: framebuffer + grafik salom ekrani ---
    println!();
    print!("[ramfb] fw_cfg orqali sozlash... ");
    match drivers::ramfb::init() {
        Some(pixels) => {
            println!(
                "OK ({}x{}, XRGB8888)",
                drivers::ramfb::FB_WIDTH,
                drivers::ramfb::FB_HEIGHT
            );
            let mut fb = Framebuffer::new(
                pixels,
                drivers::ramfb::FB_WIDTH,
                drivers::ramfb::FB_HEIGHT,
            );
            draw_boot_screen(&mut fb);
            println!("[gfx ] boot ekran chizildi");
        }
        None => {
            println!("FAILED (etc/ramfb topilmadi — `-device ramfb` qo'shildimi?)");
        }
    }

    println!();
    print!("[exc] IRQ larni globally yoqish... ");
    unsafe {
        arch::aarch64::exceptions::enable_irqs();
    }
    println!("OK");

    println!();
    println!("[boot] Faza 0-5 OK. Idle (timer ticks):");
    println!();

    loop {
        unsafe { core::arch::asm!("wfi") };
    }
}

fn draw_boot_screen(fb: &mut Framebuffer) {
    let w = fb.width();
    let h = fb.height();

    // Foni — chuqur ko'k.
    fb.clear(Color::ZAMIN_BG);

    // Yuqori chiziq.
    fb.fill_rect(0, 0, w, 4, Color::ZAMIN_FG);
    // Pastki chiziq.
    fb.fill_rect(0, h - 4, w, 4, Color::ZAMIN_FG);

    // Bosh nom — markazda, katta scale.
    let title = "ZaminOS";
    fb.draw_text_centered(80, title, Color::ZAMIN_FG, 6);

    // Subtitle.
    let subtitle = "aarch64  Rust kernel";
    fb.draw_text_centered(170, subtitle, Color::WHITE, 2);

    // Versiya.
    let version = "v0.5.0  ::  Faza 5  ::  framebuffer online";
    fb.draw_text_centered(210, version, Color::ACCENT, 1);

    // Status quti.
    let bx = 80u32;
    let by = 270u32;
    let bw = w - 160;
    let bh = 240u32;
    fb.fill_rect(bx, by, bw, bh, Color::rgb(0x14, 0x22, 0x3a));
    fb.hline(bx, by, bw, Color::ACCENT);
    fb.hline(bx, by + bh - 1, bw, Color::ACCENT);
    fb.vline(bx, by, bh, Color::ACCENT);
    fb.vline(bx + bw - 1, by, bh, Color::ACCENT);

    let lines = [
        ("[ OK ]  EL2 -> EL1 transition", Color::SUCCESS),
        ("[ OK ]  MMU: 39-bit VA, 1 GiB blocks", Color::SUCCESS),
        ("[ OK ]  Heap: 4 MiB linked-list allocator", Color::SUCCESS),
        ("[ OK ]  Exception vector + GICv2", Color::SUCCESS),
        ("[ OK ]  Generic timer @ 1 Hz", Color::SUCCESS),
        ("[ OK ]  Cooperative scheduler", Color::SUCCESS),
        ("[ OK ]  ramfb framebuffer 800x600 XRGB8888", Color::SUCCESS),
        ("", Color::WHITE),
        ("Mobile-first  konvergent OS  for aarch64", Color::ZAMIN_FG),
    ];
    let mut ty = by + 20;
    for (text, col) in lines.iter() {
        fb.draw_text(bx + 20, ty, text, *col, 2);
        ty += 22;
    }

    // Pastki ma'lumot.
    fb.draw_text_centered(h - 30, "github.com/abbosjon-dev/ZaminOS", Color::MUTED, 1);
}

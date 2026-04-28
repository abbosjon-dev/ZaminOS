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
    println!(" ZaminOS v0.6.0  —  aarch64 Rust kernel");
    println!(" Salom, dunyo! Yadro muvaffaqiyatli yuklandi.");
    println!("==============================================");
    println!();

    let el = arch::aarch64::mmu::current_el();
    println!("[boot] Joriy Exception Level: EL{}", el);
    println!("[boot] CPU: cortex-a72 (QEMU virt)");

    print!("[mmu]  Sozlash... ");
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
    println!("[demo] Heap allokator demo:");
    let boxed: Box<u64> = Box::new(0xDEADBEEFCAFEBABE);
    println!("  Box<u64> = 0x{:016X}", *boxed);
    let nums: Vec<u32> = (1..=5u32).map(|i| i * i).collect();
    println!("  Vec<u32> = {:?}", nums.as_slice());
    let s = String::from("Salom!");
    println!("  String  = \"{}\"", s);

    println!();
    print!("[exc]  Vector table... ");
    unsafe {
        arch::aarch64::exceptions::init();
    }
    println!("OK");

    print!("[gic]  Sozlash... ");
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
    print!("[ramfb] fw_cfg orqali sozlash... ");
    let fb_pixels = match drivers::ramfb::init() {
        Some(p) => {
            println!(
                "OK ({}x{}, XRGB8888)",
                drivers::ramfb::FB_WIDTH,
                drivers::ramfb::FB_HEIGHT
            );
            Some(p)
        }
        None => {
            println!("FAILED");
            None
        }
    };

    println!();
    println!("[virtio] virtio-mmio bus skanerlash...");
    let mut inputs = drivers::input::probe_inputs();
    println!("[virtio] {} ta input qurilma topildi", inputs.len());

    if let Some(pixels) = fb_pixels {
        let mut fb = Framebuffer::new(
            pixels,
            drivers::ramfb::FB_WIDTH,
            drivers::ramfb::FB_HEIGHT,
        );

        // --- Boot ekran (Faza 5 dan) ---
        draw_boot_screen(&mut fb);

        // --- Faza 6 interaktiv panel ---
        run_input_loop(&mut fb, &mut inputs);
    } else {
        println!();
        println!("[boot] Faza 0-5 OK (framebuffer yo'q).");
        loop {
            unsafe { core::arch::asm!("wfi") };
        }
    }
}

fn draw_boot_screen(fb: &mut Framebuffer) {
    let w = fb.width();
    let h = fb.height();

    fb.clear(Color::ZAMIN_BG);
    fb.fill_rect(0, 0, w, 4, Color::ZAMIN_FG);
    fb.fill_rect(0, h - 4, w, 4, Color::ZAMIN_FG);

    fb.draw_text_centered(30, "ZaminOS", Color::ZAMIN_FG, 5);
    fb.draw_text_centered(105, "aarch64  Rust kernel  -  Faza 6", Color::WHITE, 2);
    fb.draw_text_centered(140, "v0.6.0  ::  virtio-input live", Color::ACCENT, 1);

    let bx = 40u32;
    let by = 170u32;
    let bw = w - 80;
    let bh = 200u32;
    fb.fill_rect(bx, by, bw, bh, Color::rgb(0x14, 0x22, 0x3a));
    fb.hline(bx, by, bw, Color::ACCENT);
    fb.hline(bx, by + bh - 1, bw, Color::ACCENT);
    fb.vline(bx, by, bh, Color::ACCENT);
    fb.vline(bx + bw - 1, by, bh, Color::ACCENT);

    let lines = [
        "[ OK ] EL2 -> EL1, MMU, 4 MiB heap",
        "[ OK ] Exception vector + GICv2 + timer",
        "[ OK ] Cooperative scheduler",
        "[ OK ] ramfb framebuffer 800x600",
        "[ OK ] virtio-mmio bus + virtio-input",
    ];
    let mut ty = by + 18;
    for line in lines.iter() {
        fb.draw_text(bx + 18, ty, line, Color::SUCCESS, 2);
        ty += 22;
    }

    fb.draw_text_centered(h - 30, "github.com/abbosjon-dev/ZaminOS", Color::MUTED, 1);
}

const HUD_Y: u32 = 400;
const HUD_H: u32 = 170;
const TYPE_BUF_LEN: usize = 64;

fn run_input_loop(fb: &mut Framebuffer, inputs: &mut [drivers::input::Input]) -> ! {
    let w = fb.width();

    // HUD (Heads-Up Display): yozilgan matn + sichqoncha kursori.
    let hud_x = 40u32;
    let hud_w = w - 80;
    fb.fill_rect(hud_x, HUD_Y, hud_w, HUD_H, Color::rgb(0x06, 0x10, 0x1e));
    fb.hline(hud_x, HUD_Y, hud_w, Color::ZAMIN_FG);
    fb.hline(hud_x, HUD_Y + HUD_H - 1, hud_w, Color::ZAMIN_FG);
    fb.vline(hud_x, HUD_Y, HUD_H, Color::ZAMIN_FG);
    fb.vline(hud_x + hud_w - 1, HUD_Y, HUD_H, Color::ZAMIN_FG);

    fb.draw_text(hud_x + 12, HUD_Y + 12, "Klaviatura:", Color::ZAMIN_FG, 2);
    fb.draw_text(hud_x + 12, HUD_Y + 80, "Sichqoncha:", Color::ZAMIN_FG, 2);

    // IRQ larni yoqamiz — timer ticks UART ga chiqadi.
    unsafe {
        arch::aarch64::exceptions::enable_irqs();
    }

    println!();
    println!("[boot] Faza 0-6 OK. Klaviatura/sichqoncha kutilmoqda...");
    println!();

    let mut typed: String = String::new();
    let mut cursor_x: i32 = (w / 2) as i32;
    let mut cursor_y: i32 = (HUD_Y + HUD_H / 2 + 10) as i32;
    let mut prev_cx: i32 = -1;
    let mut prev_cy: i32 = -1;
    let mut event_count: u32 = 0;
    let mut dirty = true;

    loop {
        for input in inputs.iter_mut() {
            while let Some(ev) = input.pop_pending_event() {
                event_count += 1;
                handle_event(
                    ev.event_type,
                    ev.code,
                    ev.value,
                    &mut typed,
                    &mut cursor_x,
                    &mut cursor_y,
                );
                dirty = true;
            }
        }

        // Faqat holat o'zgarganda qayta chizamiz — bu screendump uchun
        // doimiy va to'liq tasvir beradi (yarim chizilgan kadrlar yo'q).
        if dirty {
            redraw_hud(fb, hud_x, hud_w, &typed, cursor_x, cursor_y, event_count);

            if prev_cx >= 0 {
                erase_cursor(fb, prev_cx, prev_cy);
            }
            draw_cursor(fb, cursor_x, cursor_y);
            prev_cx = cursor_x;
            prev_cy = cursor_y;

            dirty = false;
        }

        // Faol polling — virtio-input IRQ haligacha ulanmagan, shuning uchun
        // wfi qilmaymiz. Kichik spin-loop CPU ni biroz dam oldiradi.
        for _ in 0..10_000 {
            core::hint::spin_loop();
        }
    }
}

fn redraw_hud(
    fb: &mut Framebuffer,
    hud_x: u32,
    hud_w: u32,
    typed: &str,
    cursor_x: i32,
    cursor_y: i32,
    event_count: u32,
) {
    let kbd_y = HUD_Y + 38;
    fb.fill_rect(hud_x + 12, kbd_y, hud_w - 24, 24, Color::rgb(0x06, 0x10, 0x1e));
    let kbd_text = if typed.is_empty() {
        String::from("(klaviaturadan yozing)")
    } else {
        alloc::format!("> {}", typed)
    };
    fb.draw_text(hud_x + 12, kbd_y, &kbd_text, Color::WHITE, 2);

    let mouse_y = HUD_Y + 108;
    fb.fill_rect(hud_x + 12, mouse_y, hud_w - 24, 24, Color::rgb(0x06, 0x10, 0x1e));
    let info = alloc::format!(
        "x={:3}  y={:3}  events={}",
        cursor_x, cursor_y, event_count
    );
    fb.draw_text(hud_x + 12, mouse_y, &info, Color::ACCENT, 2);
}

fn erase_cursor(fb: &mut Framebuffer, x: i32, y: i32) {
    if x < 0 || y < 0 {
        return;
    }
    let cx = x as u32;
    let cy = y as u32;
    fb.fill_rect(cx.saturating_sub(5), cy, 11, 1, Color::ZAMIN_BG);
    fb.fill_rect(cx, cy.saturating_sub(5), 1, 11, Color::ZAMIN_BG);
}

fn handle_event(
    ev_type: u16,
    code: u16,
    value: u32,
    typed: &mut String,
    cursor_x: &mut i32,
    cursor_y: &mut i32,
) {
    match ev_type {
        x if x == drivers::input::EV_KEY => {
            // value: 1 = press, 0 = release, 2 = autorepeat
            if value == 1 {
                if let Some(ch) = drivers::input::keycode_to_char(code) {
                    if ch == '\x08' {
                        typed.pop();
                    } else if typed.len() < TYPE_BUF_LEN - 1 {
                        let _ = typed.push(ch);
                    }
                }
                println!("[input] key down: code={} ('{}')",
                    code,
                    drivers::input::keycode_to_char(code).unwrap_or('?')
                );
            }
        }
        x if x == drivers::input::EV_REL => {
            // Relative mouse motion (mouse device).
            let dv = value as i32 as i32;
            match code {
                c if c == drivers::input::REL_X => *cursor_x = (*cursor_x + dv).clamp(0, 799),
                c if c == drivers::input::REL_Y => *cursor_y = (*cursor_y + dv).clamp(0, 599),
                _ => {}
            }
        }
        x if x == drivers::input::EV_ABS => {
            // Absolute (tablet device): value diapazoni 0..=32767, ekranga moslaymiz.
            match code {
                c if c == drivers::input::ABS_X => {
                    *cursor_x = ((value as u64 * 800) / 32768) as i32;
                }
                c if c == drivers::input::ABS_Y => {
                    *cursor_y = ((value as u64 * 600) / 32768) as i32;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn draw_cursor(fb: &mut Framebuffer, x: i32, y: i32) {
    if x < 0 || y < 0 {
        return;
    }
    let cx = x as u32;
    let cy = y as u32;
    // Kichik krestik: 11x11 piksel.
    fb.fill_rect(cx.saturating_sub(5), cy, 11, 1, Color::ZAMIN_FG);
    fb.fill_rect(cx, cy.saturating_sub(5), 1, 11, Color::ZAMIN_FG);
    fb.put(cx, cy, Color::WHITE);
}

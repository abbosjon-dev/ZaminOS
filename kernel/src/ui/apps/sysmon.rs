//! System Monitor app — gauges va statistikalar.

use alloc::format;

use crate::graphics::framebuffer::{Color, FontSize, Framebuffer};
use crate::ui::Shell;

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    let big = w >= 500;
    let title_size = if big { FontSize::Px32 } else { FontSize::Px24 };
    fb.draw_text_aa(x + 20, y + 16, "System Activity", Color::ZAMIN_FG, title_size, true);

    let gauge_y = y + if big { 80 } else { 60 };
    let gauges = [
        ("Heap", shell.heap_used as u64, shell.heap_size.max(1) as u64, Color::SUCCESS),
        ("Events", shell.event_count as u64, 200, Color::ACCENT),
        ("Uptime", shell.uptime_ticks as u64, 60, Color::ZAMIN_FG),
    ];

    let gx = x + 20;
    let gw = w.saturating_sub(40);
    let gh = if big { 14 } else { 10 };
    let row_gap = if big { 26 } else { 22 };
    let mut gy = gauge_y;
    for (label, used, total, color) in gauges.iter() {
        fb.draw_text_aa(gx, gy, label, Color::WHITE, FontSize::Px16, true);

        let detail = match *label {
            "Heap" => format!("{} / {} KiB", *used / 1024, *total / 1024),
            _ => format!("{} / {}", *used, *total),
        };
        let dw = Framebuffer::measure_text_aa(&detail, FontSize::Px16, false);
        fb.draw_text_aa(gx + gw - dw, gy, &detail, Color::MUTED, FontSize::Px16, false);

        let bar_y = gy + 22;
        fb.round_rect(gx, bar_y, gw, gh, gh / 2, Color::rgb(0x1a, 0x2a, 0x44));
        let frac = if *total > 0 {
            ((*used).min(*total) * gw as u64 / *total) as u32
        } else {
            0
        };
        if frac > 0 {
            fb.round_rect(gx, bar_y, frac, gh, gh / 2, *color);
        }
        gy += row_gap + gh;
    }

    if big && h > gy - y + 100 {
        gy += 12;
        let info = [
            ("Architecture", "aarch64"),
            ("CPU", "cortex-a72 (QEMU virt)"),
            ("Boot", "EL2 -> EL1 transition"),
            ("Framebuffer", "ramfb 800x600 XRGB8888"),
            ("Input", "virtio kbd + tablet + mouse"),
        ];
        for (k, v) in info.iter() {
            fb.draw_text_aa(gx, gy, k, Color::ACCENT, FontSize::Px16, true);
            fb.draw_text_aa(gx + 180, gy, v, Color::WHITE, FontSize::Px16, false);
            gy += 22;
        }
    }
}

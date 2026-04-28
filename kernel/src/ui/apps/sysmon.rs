//! System Monitor app — grafik gauges va statistikalar.

use alloc::format;

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    let big = w >= 500;

    let title_scale = if big { 3 } else { 2 };
    fb.draw_text(x + 16, y + 14, "System", Color::ZAMIN_FG, title_scale);
    fb.draw_text(x + 16, y + 14 + (8 * title_scale + 4), "Monitor", Color::WHITE, title_scale);

    // Gauges (heap, events, uptime)
    let gauge_y = y + if big { 120 } else { 90 };
    let gauge_h = 14u32;
    let gauges = [
        ("Heap", shell.heap_used as u64, shell.heap_size.max(1) as u64, Color::SUCCESS),
        ("Events", shell.event_count as u64, 200, Color::ACCENT),
        ("Uptime", shell.uptime_ticks as u64, 60, Color::ZAMIN_FG),
    ];

    let gx = x + 16;
    let gw = w.saturating_sub(32);
    let mut gy = gauge_y;
    for (label, used, total, color) in gauges.iter() {
        fb.draw_text(gx, gy, label, Color::WHITE, 1);
        let bar_y = gy + 12;
        fb.fill_rect(gx, bar_y, gw, gauge_h, Color::rgb(0x1a, 0x2a, 0x44));
        let frac = if *total > 0 {
            ((*used).min(*total) * gw as u64 / *total) as u32
        } else {
            0
        };
        if frac > 0 {
            fb.fill_rect(gx, bar_y, frac, gauge_h, *color);
        }
        let detail = match *label {
            "Heap" => format!("{} / {} KiB", *used / 1024, *total / 1024),
            _ => format!("{} / {}", *used, *total),
        };
        fb.draw_text(
            gx + gw - (detail.len() as u32) * 8 - 4,
            gy,
            &detail,
            Color::MUTED,
            1,
        );
        gy += gauge_h + 22;
    }

    // Compact key:value list pastda
    if h > gy - y + 80 {
        let info = [
            ("Arch", "aarch64"),
            ("CPU", "cortex-a72"),
            ("Boot", "EL2 -> EL1"),
            ("FB", "ramfb 800x600"),
        ];
        let mut iy = gy + 10;
        for (k, v) in info.iter() {
            fb.draw_text(gx, iy, k, Color::ACCENT, 1);
            fb.draw_text(gx + 80, iy, v, Color::WHITE, 1);
            iy += 14;
        }
    }
}

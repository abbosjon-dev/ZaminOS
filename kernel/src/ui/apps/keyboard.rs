//! Keyboard tester app.

use alloc::format;

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, _h: u32) {
    let big = w >= 500;
    let scale = if big { 3 } else { 2 };

    fb.draw_text(x + 16, y + 14, "Keyboard", Color::ZAMIN_FG, scale);

    fb.draw_text(x + 16, y + if big { 80 } else { 60 }, "Yozilgan matn:", Color::WHITE, if big { 2 } else { 1 });

    // Input qutisi
    let box_x = x + 16;
    let box_y = y + if big { 110 } else { 80 };
    let box_w = w.saturating_sub(32);
    let box_h = if big { 36 } else { 28 };
    fb.fill_rect(box_x, box_y, box_w, box_h, Color::rgb(0x06, 0x0e, 0x1a));
    fb.hline(box_x, box_y, box_w, Color::ZAMIN_FG);
    fb.hline(box_x, box_y + box_h - 1, box_w, Color::ZAMIN_FG);
    fb.vline(box_x, box_y, box_h, Color::ZAMIN_FG);
    fb.vline(box_x + box_w - 1, box_y, box_h, Color::ZAMIN_FG);

    let display = if shell.typed.is_empty() {
        "(klaviaturadan yozing)".to_string()
    } else {
        format!("> {}", shell.typed)
    };
    let color = if shell.typed.is_empty() {
        Color::MUTED
    } else {
        Color::WHITE
    };
    let txt_scale = if big { 2 } else { 1 };
    fb.draw_text(box_x + 8, box_y + (box_h - 16) / 2, &display, color, txt_scale);

    // Stats
    let stats_y = box_y + box_h + 16;
    fb.draw_text(
        x + 16, stats_y,
        &format!("Events: {}", shell.event_count),
        Color::ACCENT, 1,
    );
    fb.draw_text(
        x + 16, stats_y + 16,
        &format!("Pointer: ({}, {})", shell.cursor_x, shell.cursor_y),
        Color::ACCENT, 1,
    );

    // Clavi tugmalarni grafik chizish (kichik QWERTY)
    if big {
        draw_keyboard_visual(fb, x + 16, stats_y + 50, w.saturating_sub(32));
    }
}

use alloc::string::ToString;

fn draw_keyboard_visual(fb: &mut Framebuffer, x: u32, y: u32, w: u32) {
    // Soddalashtirilgan QWERTY 3 satr.
    let rows = ["QWERTYUIOP", "ASDFGHJKL", "ZXCVBNM"];
    let key_w = (w / 11).max(20);
    let key_h = key_w;
    let mut row_y = y;
    for (i, row) in rows.iter().enumerate() {
        let row_w = (row.len() as u32) * (key_w + 2);
        let offset = i as u32 * (key_w / 2);
        let mut kx = x + offset + (w - row_w) / 2;
        for ch in row.chars() {
            fb.fill_rect(kx, row_y, key_w, key_h, Color::rgb(0x14, 0x22, 0x3a));
            fb.hline(kx, row_y, key_w, Color::MUTED);
            fb.hline(kx, row_y + key_h - 1, key_w, Color::MUTED);
            fb.vline(kx, row_y, key_h, Color::MUTED);
            fb.vline(kx + key_w - 1, row_y, key_h, Color::MUTED);
            let mut buf = [0u8; 4];
            let s = ch.encode_utf8(&mut buf);
            fb.draw_text(kx + (key_w - 16) / 2, row_y + (key_h - 16) / 2, s, Color::WHITE, 2);
            kx += key_w + 2;
        }
        row_y += key_h + 4;
    }
}

//! Modern app icons — har bir app uchun haqiqiy shaping.
//!
//! Eski 8x8 piktogrammalar o'rniga real shape (uy, gear, palette) kattaroq
//! o'lchamlarda procedural tarzda chiziladi.

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::apps::App;

/// Ikon foni (gradient rounded square) + glyph chizish.
/// `size` 48-128 px diapazonida yaxshi ko'rinadi.
pub fn draw_icon(fb: &mut Framebuffer, app: App, x: u32, y: u32, size: u32) {
    // Soya
    fb.shadow_rect(x, y + 4, size, size, Color::rgb(0x00, 0x00, 0x06));

    // Gradient fon, yumaloq burchaklar bilan
    let r = size / 5;
    fb.round_rect(x, y, size, size, r, app.tint_top());
    fb.vgradient(x + 2, y + size / 3, size - 4, size * 2 / 3 - 2, app.tint_top(), app.tint_bottom());

    // Glyph (oq, markazda, size'ning ~60% si)
    let g_size = size * 6 / 10;
    let g_x = x + (size - g_size) / 2;
    let g_y = y + (size - g_size) / 2;
    let cx = (x + size / 2) as i32;
    let cy = (y + size / 2) as i32;

    match app {
        App::Welcome => draw_house(fb, g_x, g_y, g_size),
        App::SysMon => draw_chart(fb, g_x, g_y, g_size),
        App::Calculator => draw_calculator(fb, g_x, g_y, g_size),
        App::Keyboard => draw_keyboard(fb, g_x, g_y, g_size),
        App::Terminal => draw_terminal(fb, g_x, g_y, g_size),
        App::Paint => draw_palette(fb, cx, cy, g_size / 2),
        App::Files => draw_folder(fb, g_x, g_y, g_size),
        App::Music => draw_note(fb, g_x, g_y, g_size),
        App::Clock => draw_clock(fb, cx, cy, g_size / 2),
        App::Settings => draw_gear(fb, cx, cy, g_size / 2),
    }
}

fn draw_house(fb: &mut Framebuffer, x: u32, y: u32, size: u32) {
    let c = Color::WHITE;
    // Tom (uchburchak)
    let mid = x + size / 2;
    let tom_h = size / 3;
    for j in 0..tom_h {
        let half_w = (j * (size / 2) / tom_h).max(1);
        fb.fill_rect(mid - half_w, y + j, 2 * half_w, 1, c);
    }
    // Tana (kvadrat)
    let body_y = y + tom_h;
    let body_h = size - tom_h;
    let body_x = x + size / 6;
    let body_w = size - size / 3;
    fb.fill_rect(body_x, body_y, body_w, 2, c);
    fb.fill_rect(body_x, body_y, 2, body_h, c);
    fb.fill_rect(body_x + body_w - 2, body_y, 2, body_h, c);
    fb.fill_rect(body_x, y + size - 2, body_w, 2, c);
    // Eshik (markazda)
    let door_w = body_w / 4;
    let door_h = body_h / 2;
    let door_x = body_x + (body_w - door_w) / 2;
    fb.fill_rect(door_x, y + size - door_h, door_w, door_h, c);
}

fn draw_chart(fb: &mut Framebuffer, x: u32, y: u32, size: u32) {
    let c = Color::WHITE;
    // Pastki chiziq
    fb.fill_rect(x, y + size - 3, size, 3, c);
    // Vertikal chiziq
    fb.fill_rect(x, y, 3, size, c);
    // Bars
    let bar_w = (size - 12) / 4;
    let mut bx = x + 8;
    let heights = [size * 4 / 10, size * 7 / 10, size * 5 / 10, size * 8 / 10];
    for h in heights.iter() {
        fb.round_rect(bx, y + size - 3 - h, bar_w, *h, 2, c);
        bx += bar_w + 2;
    }
}

fn draw_calculator(fb: &mut Framebuffer, x: u32, y: u32, size: u32) {
    let c = Color::WHITE;
    if size < 36 {
        // Kichik o'lchamda — sodda kvadrat + display chizig'i
        let r = size / 6;
        fb.round_rect(x + 1, y + 1, size - 2, size - 2, r, c);
        let inner = Color::rgb(0xff, 0x9f, 0x0a);
        let pad = (size / 8).max(1);
        fb.round_rect(x + pad, y + pad, size - 2 * pad, size - 2 * pad, r.saturating_sub(1), inner);
        // Bitta gorizontal chiziq display sifatida
        fb.fill_rect(x + pad + 1, y + size / 3, size - 2 * pad - 2, 2, c);
        return;
    }
    let r = size / 8;
    fb.round_rect(x, y, size, size, r, c);
    let inner = Color::rgb(0xff, 0x9f, 0x0a);
    fb.round_rect(x + 3, y + 3, size - 6, size - 6, r.saturating_sub(2), inner);
    let disp_h = size / 4;
    fb.round_rect(x + 6, y + 6, size - 12, disp_h, 3, Color::rgb(0x14, 0x18, 0x28));
    let btn_y = y + 6 + disp_h + 4;
    let btn_area_h = (size).saturating_sub(12 + disp_h + 8);
    let cols = 3u32;
    let rows = 3u32;
    let bw = (size - 12) / cols;
    let bh = (btn_area_h / rows).max(2);
    for rr in 0..rows {
        for col in 0..cols {
            let bx = x + 6 + col * bw + 2;
            let by = btn_y + rr * bh + 2;
            if bw > 4 && bh > 4 {
                fb.round_rect(bx, by, bw - 4, bh - 4, 2, c);
            }
        }
    }
}

fn draw_keyboard(fb: &mut Framebuffer, x: u32, y: u32, size: u32) {
    let c = Color::WHITE;
    let r = (size / 12).max(2);
    fb.round_rect(x, y + size / 4, size, size / 2, r, c);
    let inner = Color::rgb(0x42, 0xa5, 0xf5);
    fb.round_rect(x + 3, y + size / 4 + 3, size - 6, size / 2 - 6, r.saturating_sub(1), inner);
    if size < 36 {
        return;
    }
    let key_size = (size / 12).max(3);
    let gap = (size / 20).max(1);
    for rr in 0..3u32 {
        let row_y = y + size / 4 + 8 + rr * (key_size + gap);
        let n_keys: u32 = (7 - rr).min((size / (key_size + gap)).max(1));
        let row_w = n_keys * key_size + n_keys.saturating_sub(1) * gap;
        let row_x = x + size.saturating_sub(row_w) / 2;
        for k in 0..n_keys {
            fb.round_rect(row_x + k * (key_size + gap), row_y, key_size, key_size, key_size / 4, c);
        }
    }
}

fn draw_terminal(fb: &mut Framebuffer, x: u32, y: u32, size: u32) {
    let c = Color::WHITE;
    let r = (size / 12).max(2);
    fb.round_rect(x, y, size, size, r, c);
    let bar_h = (size / 6).max(3);
    fb.fill_rect(x, y, size, bar_h, c);
    if size >= 36 {
        let dot_y = y + bar_h / 2;
        let dot_r = (bar_h / 4).max(2);
        fb.fill_circle_aa((x + bar_h - 2) as i32, dot_y as i32, dot_r, Color::rgb(0xef, 0x53, 0x50));
        fb.fill_circle_aa((x + 2 * bar_h - 4) as i32, dot_y as i32, dot_r, Color::rgb(0xfd, 0xc4, 0x35));
        fb.fill_circle_aa((x + 3 * bar_h - 6) as i32, dot_y as i32, dot_r, Color::rgb(0x66, 0xbb, 0x6a));
    }
    let body_y = y + bar_h;
    let body_h = size.saturating_sub(bar_h);
    fb.fill_rect(x + 2, body_y, size - 4, body_h.saturating_sub(2), Color::rgb(0x14, 0x18, 0x28));
    if size >= 28 {
        let prompt_x = x + 6;
        let prompt_y = body_y + body_h / 3;
        let p_size = (size / 8).max(3);
        fb.fill_rect(prompt_x, prompt_y, p_size, 2, Color::SUCCESS);
        fb.fill_rect(prompt_x + p_size - 2, prompt_y.saturating_sub(2), 2, 6, Color::SUCCESS);
        fb.fill_rect(prompt_x, prompt_y + 4, p_size, 2, Color::SUCCESS);
        fb.fill_rect(prompt_x + p_size + 4, prompt_y, (p_size / 2).max(2), 6, Color::SUCCESS);
    }
}

fn draw_palette(fb: &mut Framebuffer, cx: i32, cy: i32, r: u32) {
    // Palitra: katta disk + kichik rang dotlari + brush
    let palette_colors = [
        Color::rgb(0xff, 0xc7, 0x4c),
        Color::rgb(0x42, 0xa5, 0xf5),
        Color::rgb(0x66, 0xbb, 0x6a),
        Color::rgb(0xef, 0x53, 0x50),
        Color::rgb(0xab, 0x47, 0xbc),
    ];
    fb.fill_circle_aa(cx, cy, r, Color::WHITE);
    fb.fill_circle_aa(cx, cy, r * 7 / 10, Color::rgb(0xab, 0x47, 0xbc));

    // Rang dotlari (5 ta yarim doira atrofida)
    for (i, c) in palette_colors.iter().enumerate() {
        let ang = -3.14159 * 0.6 + (i as f32) * 3.14159 * 0.3;
        let dx = (cos_a(ang) * (r as f32) * 0.7) as i32;
        let dy = (sin_a(ang) * (r as f32) * 0.7) as i32;
        fb.fill_circle_aa(cx + dx, cy + dy, r / 5, *c);
    }
    // Brush hole
    fb.fill_circle_aa(cx + (r as i32) / 2, cy + (r as i32) / 2, r / 6, Color::rgb(0xab, 0x47, 0xbc));
}

fn draw_folder(fb: &mut Framebuffer, x: u32, y: u32, size: u32) {
    let c = Color::WHITE;
    // Tab (yuqori chap)
    let tab_w = size / 2;
    let tab_h = size / 6;
    fb.round_rect(x, y + size / 5, tab_w, tab_h, 3, c);
    // Body
    let body_y = y + size / 5 + tab_h - 4;
    let body_h = size - (body_y - y) - size / 12;
    fb.round_rect(x, body_y, size, body_h, 4, c);
    // Inner darken
    fb.round_rect(x + 3, body_y + 3, size - 6, body_h - 6, 3, Color::rgb(0x29, 0xb6, 0xf6));
    // Document chiziqlari
    let line_w = size * 6 / 10;
    let line_x = x + (size - line_w) / 2;
    for i in 0..3 {
        fb.fill_rect(line_x, body_y + body_h / 4 + i * 8, line_w, 2, c);
    }
}

fn draw_note(fb: &mut Framebuffer, x: u32, y: u32, size: u32) {
    let c = Color::WHITE;
    // Stem (vertical line)
    let stem_x = x + size * 7 / 10;
    fb.fill_rect(stem_x, y + size / 8, 4, size * 3 / 4, c);
    // Flag (yuqorida)
    fb.round_rect(stem_x, y + size / 8, size / 3, size / 4, 2, c);
    // Note head (pastida)
    let head_cx = (x + size / 4) as i32;
    let head_cy = (y + size * 4 / 5) as i32;
    let head_r = size / 5;
    fb.fill_circle_aa(head_cx, head_cy, head_r, c);
    // Stem to head
    fb.fill_rect(head_cx as u32 + head_r - 2, y + size / 8, 4, size * 4 / 5 - size / 8, c);
}

fn draw_clock(fb: &mut Framebuffer, cx: i32, cy: i32, r: u32) {
    // Tashqi halqa
    fb.ring(cx, cy, r, 3, Color::WHITE);
    fb.fill_circle_aa(cx, cy, r.saturating_sub(3), Color::rgb(0xef, 0x53, 0x50));
    // 12, 3, 6, 9 markerlari
    let r_ = r as i32;
    fb.fill_rect((cx - 1) as u32, (cy - r_ + 4) as u32, 2, 4, Color::WHITE);
    fb.fill_rect((cx + r_ - 8) as u32, (cy - 1) as u32, 4, 2, Color::WHITE);
    fb.fill_rect((cx - 1) as u32, (cy + r_ - 8) as u32, 2, 4, Color::WHITE);
    fb.fill_rect((cx - r_ + 4) as u32, (cy - 1) as u32, 4, 2, Color::WHITE);
    // Hands
    fb.fill_rect((cx - 1) as u32, (cy - r_ * 6 / 10) as u32, 3, (r_ * 6 / 10) as u32, Color::WHITE);
    fb.fill_rect(cx as u32, cy as u32, (r_ * 5 / 10) as u32, 2, Color::WHITE);
    // Center dot
    fb.fill_circle_aa(cx, cy, 3, Color::WHITE);
}

fn draw_gear(fb: &mut Framebuffer, cx: i32, cy: i32, r: u32) {
    let teeth_count = 8;
    let r_inner = r * 7 / 10;
    let r_outer = r;
    // Tashqi gear shaklini yasash: 8 ta tooth
    for i in 0..teeth_count {
        let ang = (i as f32) * 2.0 * 3.14159 / (teeth_count as f32);
        let cs = cos_a(ang);
        let sn = sin_a(ang);
        let tx = cx + (cs * r_outer as f32) as i32;
        let ty = cy + (sn * r_outer as f32) as i32;
        // Tooth — kichik kvadrat
        let tooth_size = r_outer / 4;
        fb.fill_rect(
            (tx - tooth_size as i32 / 2).max(0) as u32,
            (ty - tooth_size as i32 / 2).max(0) as u32,
            tooth_size, tooth_size,
            Color::WHITE,
        );
    }
    // Asosiy disk
    fb.fill_circle_aa(cx, cy, r_inner, Color::WHITE);
    // Markazda hole
    fb.fill_circle_aa(cx, cy, r_inner / 2, Color::rgb(0x78, 0x90, 0x9c));
    // Hole ichida tag
    fb.fill_circle_aa(cx, cy, r_inner / 5, Color::WHITE);
}

// Sodda Taylor sin/cos.
fn sin_a(mut x: f32) -> f32 {
    let two_pi = 2.0 * 3.14159;
    while x > 3.14159 { x -= two_pi; }
    while x < -3.14159 { x += two_pi; }
    let x2 = x * x;
    x * (1.0 - x2 / 6.0 + x2 * x2 / 120.0)
}

fn cos_a(x: f32) -> f32 {
    sin_a(x + 3.14159 / 2.0)
}

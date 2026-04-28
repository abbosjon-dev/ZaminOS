//! Paint app — sichqoncha bilan chizish.

use alloc::vec::Vec;

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

#[derive(Clone, Copy)]
struct Stroke {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: u32,
}

pub struct PaintState {
    strokes: Vec<Stroke>,
    palette_idx: usize,
}

const PALETTE: [Color; 6] = [
    Color::rgb(0xfd, 0xc4, 0x35), // gold
    Color::rgb(0x29, 0xb6, 0xf6), // blue
    Color::rgb(0x66, 0xbb, 0x6a), // green
    Color::rgb(0xef, 0x53, 0x50), // red
    Color::rgb(0xab, 0x47, 0xbc), // purple
    Color::rgb(0xff, 0xff, 0xff), // white
];

impl PaintState {
    pub fn new() -> Self {
        Self {
            strokes: Vec::new(),
            palette_idx: 0,
        }
    }

    pub fn on_pointer(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        self.strokes.push(Stroke {
            x0,
            y0,
            x1,
            y1,
            color: PALETTE[self.palette_idx].0,
        });
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.strokes.clear();
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // Title bar
    fb.fill_rect(x, y, w, 22, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 8, y + 6, "Paint  -  Sichqoncha bilan chizing", Color::ZAMIN_FG, 1);

    // Palette (yuqori)
    let pal_y = y + 22;
    let pal_h = 24u32;
    fb.fill_rect(x, pal_y, w, pal_h, Color::rgb(0x0a, 0x14, 0x26));
    let sw_w = 22u32;
    let mut px = x + 8;
    for (i, c) in PALETTE.iter().enumerate() {
        let active = i == shell.paint.palette_idx;
        if active {
            fb.fill_rect(px - 2, pal_y + 2, sw_w + 4, sw_w - 4 + 4, Color::WHITE);
        }
        fb.fill_rect(px, pal_y + 4, sw_w, sw_w - 4, *c);
        px += sw_w + 6;
    }

    // Canvas
    let cv_y = pal_y + pal_h + 2;
    let cv_h = (y + h).saturating_sub(cv_y);
    fb.fill_rect(x, cv_y, w, cv_h, Color::WHITE);

    // Stroke'larni chizish (klipping bilan)
    for s in shell.paint.strokes.iter() {
        // 3-pikselli yo'g'on chiziq imitatsiyasi
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                draw_line(
                    fb,
                    s.x0 + dx,
                    s.y0 + dy,
                    s.x1 + dx,
                    s.y1 + dy,
                    Color(s.color),
                    x,
                    cv_y,
                    w,
                    cv_h,
                );
            }
        }
    }

    // Pastki ko'rsatma
    let hint_y = y + h - 14;
    fb.fill_rect(x, hint_y, w, 14, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 6, hint_y + 2, "Sichqoncha = chizish, palette swatch'ni bosing", Color::MUTED, 1);
}

#[allow(clippy::too_many_arguments)]
fn draw_line(
    fb: &mut Framebuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
    cx: u32,
    cy: u32,
    cw: u32,
    ch: u32,
) {
    // Bresenham
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let cx_end = cx + cw;
    let cy_end = cy + ch;

    loop {
        if x >= cx as i32 && (x as u32) < cx_end && y >= cy as i32 && (y as u32) < cy_end {
            fb.put(x as u32, y as u32, color);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

//! Clock app — analog soat sirti.

use alloc::format;

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_rect(x, y, w, 22, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 8, y + 6, "Clock", Color::ZAMIN_FG, 1);

    // Soat dial diametri
    let cx = (x + w / 2) as i32;
    let cy = (y + 22 + (h - 22) / 2) as i32;
    let r = (w.min(h - 22) / 2 - 16) as i32;
    if r < 30 {
        fb.draw_text(x + 6, y + 30, "Juda kichik joy", Color::MUTED, 1);
        return;
    }

    // Tashqi halqa
    draw_circle(fb, cx, cy, r as u32, Color::ZAMIN_FG);
    draw_circle(fb, cx, cy, (r - 1) as u32, Color::ZAMIN_FG);

    // Markaz
    fb.fill_rect((cx - 2) as u32, (cy - 2) as u32, 5, 5, Color::ZAMIN_FG);

    // Soat raqamlari (12, 3, 6, 9)
    fb.draw_text((cx - 6) as u32, (cy - r + 4) as u32, "12", Color::WHITE, 1);
    fb.draw_text((cx + r - 12) as u32, (cy - 4) as u32, "3", Color::WHITE, 1);
    fb.draw_text((cx - 4) as u32, (cy + r - 14) as u32, "6", Color::WHITE, 1);
    fb.draw_text((cx - r + 6) as u32, (cy - 4) as u32, "9", Color::WHITE, 1);

    // Tick belgilar
    for i in 0..12 {
        let ang = (i as f32) * core::f32::consts::PI / 6.0 - core::f32::consts::FRAC_PI_2;
        let cos = cos_approx(ang);
        let sin = sin_approx(ang);
        let r1 = r - 6;
        let r2 = r - 2;
        let x1 = cx + (cos * r1 as f32) as i32;
        let y1 = cy + (sin * r1 as f32) as i32;
        let x2 = cx + (cos * r2 as f32) as i32;
        let y2 = cy + (sin * r2 as f32) as i32;
        line(fb, x1, y1, x2, y2, Color::MUTED);
    }

    // Vaqt: uptime asosida soat/minut/sekund.
    let total = shell.uptime_ticks as i32;
    let s = total % 60;
    let m = (total / 60) % 60;
    let hh = (total / 3600) % 12;

    let sec_ang = (s as f32) * core::f32::consts::PI / 30.0 - core::f32::consts::FRAC_PI_2;
    let min_ang = (m as f32) * core::f32::consts::PI / 30.0 - core::f32::consts::FRAC_PI_2;
    let hr_ang = ((hh as f32) + (m as f32) / 60.0) * core::f32::consts::PI / 6.0 - core::f32::consts::FRAC_PI_2;

    // Hour
    {
        let len = (r as f32 * 0.5) as i32;
        let x2 = cx + (cos_approx(hr_ang) * len as f32) as i32;
        let y2 = cy + (sin_approx(hr_ang) * len as f32) as i32;
        thick_line(fb, cx, cy, x2, y2, Color::ZAMIN_FG);
    }
    // Minute
    {
        let len = (r as f32 * 0.7) as i32;
        let x2 = cx + (cos_approx(min_ang) * len as f32) as i32;
        let y2 = cy + (sin_approx(min_ang) * len as f32) as i32;
        thick_line(fb, cx, cy, x2, y2, Color::WHITE);
    }
    // Second
    {
        let len = (r as f32 * 0.85) as i32;
        let x2 = cx + (cos_approx(sec_ang) * len as f32) as i32;
        let y2 = cy + (sin_approx(sec_ang) * len as f32) as i32;
        line(fb, cx, cy, x2, y2, Color::ACCENT);
    }

    // Pastdagi raqamli soat
    let digital = format!("{:02}:{:02}:{:02}", hh, m, s);
    let dx = (x + w / 2).saturating_sub((digital.len() as u32 * 8) / 2);
    fb.draw_text(dx, y + h - 18, &digital, Color::ACCENT, 1);
}

fn line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let w = fb.width();
    let h = fb.height();
    loop {
        if x >= 0 && (x as u32) < w && y >= 0 && (y as u32) < h {
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

fn thick_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            line(fb, x0 + dx, y0 + dy, x1 + dx, y1 + dy, color);
        }
    }
}

fn draw_circle(fb: &mut Framebuffer, cx: i32, cy: i32, r: u32, color: Color) {
    let mut x = r as i32;
    let mut y = 0i32;
    let mut err = 0i32;
    let w = fb.width();
    let h = fb.height();
    let mut put = |px: i32, py: i32| {
        if px >= 0 && (px as u32) < w && py >= 0 && (py as u32) < h {
            fb.put(px as u32, py as u32, color);
        }
    };
    while x >= y {
        put(cx + x, cy + y);
        put(cx + y, cy + x);
        put(cx - y, cy + x);
        put(cx - x, cy + y);
        put(cx - x, cy - y);
        put(cx - y, cy - x);
        put(cx + y, cy - x);
        put(cx + x, cy - y);
        if err <= 0 {
            y += 1;
            err += 2 * y + 1;
        }
        if err > 0 {
            x -= 1;
            err -= 2 * x + 1;
        }
    }
}

// Taylor series ravishda sin/cos taxminiy hisoblash (no_std libm yo'q).
// |x| <= pi diapazonda etarli aniqlik.
fn sin_approx(mut x: f32) -> f32 {
    let two_pi = 2.0 * core::f32::consts::PI;
    while x > core::f32::consts::PI {
        x -= two_pi;
    }
    while x < -core::f32::consts::PI {
        x += two_pi;
    }
    let x2 = x * x;
    x * (1.0 - x2 / 6.0 + x2 * x2 / 120.0 - x2 * x2 * x2 / 5040.0)
}

fn cos_approx(x: f32) -> f32 {
    sin_approx(x + core::f32::consts::FRAC_PI_2)
}

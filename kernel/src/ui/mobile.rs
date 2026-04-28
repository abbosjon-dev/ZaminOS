//! Android uslubli mobile layout.
//!
//! - Status bar (yuqorida ingichka, signal/wifi/batareya)
//! - Home screen: katta soat widget, weather widget, music widget, search bar
//! - Pastda dock: 3 ta tezkor app + 4-chi "menu" tugmasi
//! - Menu bosilsa — App drawer (4-column grid)
//! - InApp ko'rinishi: app contenti + pastki bottom navigation

use alloc::format;

use crate::graphics::framebuffer::{Color, FontSize, Framebuffer};
use crate::ui::apps::App;
use crate::ui::{
    draw_app_icon_modern, draw_battery_icon, draw_wifi_icon, MobileView, Shell,
};

const PHONE_W: u32 = 360;
const PHONE_H: u32 = 580;

pub fn draw(shell: &Shell, fb: &mut Framebuffer) {
    let w = fb.width();
    let h = fb.height();

    // ---- Workspace fon (gradient + accent) ----
    fb.vgradient(0, 0, w, h, Color::rgb(0x18, 0x1f, 0x4a), Color::rgb(0x05, 0x0a, 0x1c));

    // Yon panellar (chap/o'ng)
    draw_side_caption(shell, fb, w, h);

    // ---- Telefon shakli ----
    let px = (w - PHONE_W) / 2;
    let py = (h - PHONE_H) / 2;

    // Bezel (qora chiziqcha + soya)
    fb.shadow_rect(px - 8, py - 8 + 8, PHONE_W + 16, PHONE_H + 16, Color::rgb(0x00, 0x00, 0x04));
    fb.round_rect(px - 8, py - 8, PHONE_W + 16, PHONE_H + 16, 28, Color::rgb(0x14, 0x14, 0x1c));

    // Ekran fon
    fb.round_rect(px, py, PHONE_W, PHONE_H, 22, Color::rgb(0x06, 0x09, 0x18));

    // Wallpaper inside (gradient)
    draw_phone_wallpaper(fb, px, py, PHONE_W, PHONE_H);

    // Notch
    let notch_w = 100u32;
    fb.round_rect(px + (PHONE_W - notch_w) / 2, py - 2, notch_w, 18, 8, Color::rgb(0x10, 0x10, 0x16));
    // Camera dot
    fb.fill_circle(
        (px + (PHONE_W + notch_w) / 2 - 14) as i32,
        (py + 7) as i32,
        3, Color::rgb(0x40, 0x40, 0x52),
    );

    // Status bar
    draw_status_bar(fb, px, py, PHONE_W);

    // Content + dock (view ga qarab)
    match shell.mobile_view {
        MobileView::Home => {
            draw_home(shell, fb, px, py + 32, PHONE_W, PHONE_H - 32 - 84);
            draw_dock(shell, fb, px, py + PHONE_H - 84, PHONE_W);
        }
        MobileView::AppDrawer => {
            draw_app_drawer(shell, fb, px, py + 32, PHONE_W, PHONE_H - 32);
        }
        MobileView::InApp => {
            draw_in_app(shell, fb, px, py + 32, PHONE_W, PHONE_H - 32 - 50);
            draw_in_app_nav(shell, fb, px, py + PHONE_H - 50, PHONE_W);
        }
    }

    // Home indicator (pastki polosa)
    let hi_w = 110u32;
    fb.round_rect(
        px + (PHONE_W - hi_w) / 2,
        py + PHONE_H - 8,
        hi_w, 4, 2,
        Color::rgb(0xff, 0xff, 0xff),
    );
}

fn draw_side_caption(shell: &Shell, fb: &mut Framebuffer, w: u32, _h: u32) {
    fb.draw_text_aa(20, 80, "ZaminOS", Color::ZAMIN_FG, FontSize::Px32, true);
    fb.draw_text_aa(20, 122, "Mobile", Color::WHITE, FontSize::Px24, false);
    fb.fill_rect(20, 154, 80, 2, Color::ACCENT);
    fb.draw_text_aa(20, 168, "Esc - Desktop", Color::MUTED, FontSize::Px16, false);
    fb.draw_text_aa(20, 188, "F2  - App drawer", Color::MUTED, FontSize::Px16, false);
    fb.draw_text_aa(20, 208, "F1  - Home", Color::MUTED, FontSize::Px16, false);

    let r_x = w - 200;
    fb.draw_text_aa(r_x, 80, "Konvergent", Color::ZAMIN_FG, FontSize::Px24, true);
    fb.fill_rect(r_x, 110, 130, 2, Color::ACCENT);
    fb.draw_text_aa(r_x, 122, "Bitta OS,", Color::WHITE, FontSize::Px16, false);
    fb.draw_text_aa(r_x, 142, "Telefon -> PC.", Color::WHITE, FontSize::Px16, false);
    fb.draw_text_aa(r_x, 174, &format!("Uptime: {} s", shell.uptime_ticks), Color::MUTED, FontSize::Px16, false);
    fb.draw_text_aa(r_x, 194, &format!("Events: {}", shell.event_count), Color::MUTED, FontSize::Px16, false);
}

fn draw_phone_wallpaper(fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.vgradient(x, y, w, h, Color::rgb(0x1a, 0x14, 0x46), Color::rgb(0x06, 0x0c, 0x24));
    // Diagonal accent (faint)
    let cx = (x + w * 3 / 4) as i32;
    let cy = (y + h / 4) as i32;
    for r in 0..50 {
        let alpha = 50 - r;
        let _ = alpha;
    }
    // Quick accent dot in upper right
    for r in 0..80u32 {
        let intensity = (80 - r) / 4;
        if intensity == 0 { continue; }
        for ang in 0..360 {
            let theta = ang as f32 * 3.14159 / 180.0;
            let dx = (r as f32 * cos(theta)) as i32;
            let dy = (r as f32 * sin(theta)) as i32;
            let px = cx + dx;
            let py = cy + dy;
            if px >= x as i32 && (px as u32) < x + w && py >= y as i32 && (py as u32) < y + h {
                let idx = (py as u32 * fb.width() + px as u32) as usize;
                let p = fb.raw_pixel(idx);
                let r0 = (p >> 16) & 0xff;
                let g0 = (p >> 8) & 0xff;
                let b0 = p & 0xff;
                let r1 = (r0 + intensity).min(255);
                let g1 = (g0 + intensity / 2).min(255);
                let b1 = (b0 + intensity).min(255);
                fb.set_raw_pixel(idx, (r1 << 16) | (g1 << 8) | b1);
            }
        }
    }
}

fn cos(x: f32) -> f32 {
    let x2 = x * x;
    1.0 - x2 / 2.0 + x2 * x2 / 24.0 - x2 * x2 * x2 / 720.0
}
fn sin(mut x: f32) -> f32 {
    let two_pi = 2.0 * 3.14159;
    while x > 3.14159 { x -= two_pi; }
    while x < -3.14159 { x += two_pi; }
    let x2 = x * x;
    x * (1.0 - x2 / 6.0 + x2 * x2 / 120.0)
}

fn draw_status_bar(fb: &mut Framebuffer, x: u32, y: u32, w: u32) {
    // Vaqt chap
    fb.draw_text_aa(x + 18, y + 6, "9:41", Color::WHITE, FontSize::Px16, true);
    // O'ng tomonda signal/wifi/batareya
    let mut sx = x + w - 18;
    sx = sx.saturating_sub(28);
    draw_battery_icon(fb, sx, y + 8);
    sx = sx.saturating_sub(20);
    draw_wifi_icon(fb, sx, y + 8);
    sx = sx.saturating_sub(28);
    fb.draw_text_aa(sx, y + 6, "5G", Color::WHITE, FontSize::Px16, true);
}

fn draw_home(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    let pad = 16u32;

    // ---- Katta soat widget ----
    let clock_y = y + 8;
    let total = shell.uptime_ticks;
    let hh = (total / 3600) % 24;
    let mm = (total / 60) % 60;
    let time_str = format!("{:02}:{:02}", hh, mm);
    fb.draw_text_aa_centered(x + w / 2, clock_y, &time_str, Color::WHITE, FontSize::Px32, true);
    let date_str = "Dushanba, 28 Apr";
    fb.draw_text_aa_centered(x + w / 2, clock_y + 38, date_str, Color::MUTED, FontSize::Px16, false);

    // ---- Search bar ----
    let sb_y = clock_y + 70;
    fb.round_rect(x + pad, sb_y, w - 2 * pad, 36, 18, Color::rgb(0x18, 0x1f, 0x36));
    fb.fill_circle((x + pad + 18) as i32, (sb_y + 18) as i32, 6, Color::rgb(0x80, 0x80, 0x90));
    fb.fill_rect(x + pad + 22, sb_y + 21, 8, 2, Color::rgb(0x80, 0x80, 0x90));
    fb.draw_text_aa(x + pad + 36, sb_y + 9, "Search...", Color::MUTED, FontSize::Px16, false);

    // ---- Weather widget (chap) + Music widget (o'ng) ----
    let wid_y = sb_y + 50;
    let wid_w = (w - 3 * pad) / 2;
    let wid_h = 100u32;

    // Weather
    fb.round_rect(x + pad, wid_y, wid_w, wid_h, 16, Color::rgb(0x29, 0x39, 0x6b));
    fb.draw_text_aa(x + pad + 12, wid_y + 8, "Toshkent", Color::WHITE, FontSize::Px16, true);
    fb.draw_text_aa(x + pad + 12, wid_y + 36, "+22", Color::WHITE, FontSize::Px32, true);
    fb.draw_text_aa(x + pad + 12, wid_y + 76, "Quyoshli", Color::ACCENT, FontSize::Px16, false);
    // Sun icon
    fb.fill_circle((x + pad + wid_w - 28) as i32, (wid_y + 28) as i32, 14, Color::rgb(0xff, 0xc7, 0x4c));
    for i in 0..8 {
        let ang = i as f32 * core::f32::consts::PI / 4.0;
        let cs = cos(ang);
        let sn = sin(ang);
        let cx = (x + pad + wid_w - 28) as i32 + (cs * 22.0) as i32;
        let cy = (wid_y + 28) as i32 + (sn * 22.0) as i32;
        fb.fill_circle(cx, cy, 2, Color::rgb(0xff, 0xc7, 0x4c));
    }

    // Music
    let mx = x + pad + wid_w + pad;
    fb.round_rect(mx, wid_y, wid_w, wid_h, 16, Color::rgb(0xec, 0x40, 0x7a));
    fb.draw_text_aa(mx + 12, wid_y + 8, "Now Playing", Color::WHITE, FontSize::Px16, true);
    fb.draw_text_aa(mx + 12, wid_y + 32, "ZaminOS", Color::WHITE, FontSize::Px20, true);
    fb.draw_text_aa(mx + 12, wid_y + 56, "Boot Theme", Color::WHITE, FontSize::Px16, false);
    // Play tugmasi
    let pb_x = mx + wid_w - 36;
    let pb_y = wid_y + wid_h - 36;
    fb.fill_circle((pb_x + 14) as i32, (pb_y + 14) as i32, 14, Color::WHITE);
    // Play uchburchak
    let cx = pb_x + 14;
    let cy = pb_y + 14;
    for i in 0..6u32 {
        fb.fill_rect(cx as u32 - 3 + i / 2, cy as u32 - i, 1, 2 * i, Color::rgb(0xec, 0x40, 0x7a));
    }

    // ---- Activity widget (pastda keng) ----
    let act_y = wid_y + wid_h + 16;
    let act_h = h.saturating_sub(act_y - y).saturating_sub(8);
    if act_h > 60 {
        fb.round_rect(x + pad, act_y, w - 2 * pad, act_h.min(120), 16, Color::rgb(0x18, 0x1f, 0x36));
        fb.draw_text_aa(x + pad + 12, act_y + 8, "Tizim", Color::WHITE, FontSize::Px16, true);
        // Heap mini progress
        let (size, used, _) = (shell.heap_size, shell.heap_used, 0u32);
        let pct = if size > 0 { (used * 100 / size) as u32 } else { 0 };
        fb.draw_text_aa(x + pad + 12, act_y + 32, &format!("Heap: {}%", pct), Color::ACCENT, FontSize::Px16, false);
        let bar_x = x + pad + 12;
        let bar_y = act_y + 56;
        let bar_w = w - 2 * pad - 24;
        fb.round_rect(bar_x, bar_y, bar_w, 8, 4, Color::rgb(0x30, 0x36, 0x4a));
        let frac = bar_w * pct / 100;
        if frac > 0 {
            fb.round_rect(bar_x, bar_y, frac, 8, 4, Color::rgb(0x4c, 0xaf, 0x50));
        }
        if act_h >= 100 {
            fb.draw_text_aa(x + pad + 12, act_y + 76, &format!("Uptime: {}s · Events: {}", shell.uptime_ticks, shell.event_count), Color::MUTED, FontSize::Px16, false);
        }
    }
}

fn draw_dock(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32) {
    // Pastki dock — 3 ta dastlabki app + menu (Apps) tugmasi
    let pad = 16u32;
    let dock_h = 76u32;
    fb.round_rect(x + pad, y, w - 2 * pad, dock_h, 22, Color::rgb(0x14, 0x1a, 0x2c));

    let dock_apps = [App::Welcome, App::Music, App::Clock]; // 3 ta tezkor
    let icon = 48u32;
    let n = 4u32; // 3 + Apps menu
    let gap = (w - 2 * pad - n * icon) / (n + 1);
    let iy = y + (dock_h - icon) / 2;
    let mut ix = x + pad + gap;
    for app in dock_apps.iter() {
        draw_app_icon_modern(fb, *app, ix, iy, icon);
        ix += icon + gap;
    }
    // Apps menu tugmasi (4-chi) — Android uslubidagi 9 nuqta
    fb.round_rect(ix, iy, icon, icon, 12, Color::rgb(0x35, 0x3b, 0x52));
    let dot_size = 5u32;
    let dot_gap = 6u32;
    let grid_w = 3 * dot_size + 2 * dot_gap;
    let dx0 = ix + (icon - grid_w) / 2;
    let dy0 = iy + (icon - grid_w) / 2;
    for r in 0..3u32 {
        for c in 0..3u32 {
            fb.fill_rect(
                dx0 + c * (dot_size + dot_gap),
                dy0 + r * (dot_size + dot_gap),
                dot_size, dot_size,
                Color::WHITE,
            );
        }
    }
}

fn draw_app_drawer(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // Sarlavha
    fb.draw_text_aa(x + 16, y + 8, "Apps", Color::WHITE, FontSize::Px24, true);
    fb.draw_text_aa(x + 16, y + 38, "Hammasi", Color::MUTED, FontSize::Px16, false);

    // 4 ustunli grid
    let grid_y = y + 70;
    let cols = 4u32;
    let icon = 56u32;
    let gap = 12u32;
    let total_w = cols * icon + (cols - 1) * gap;
    let start_x = x + (w - total_w) / 2;

    let apps = App::all();
    for (idx, app) in apps.iter().enumerate() {
        let r = (idx as u32) / cols;
        let c = (idx as u32) % cols;
        let ix = start_x + c * (icon + gap);
        let iy = grid_y + r * (icon + 28 + 14);
        if iy + icon + 28 > y + h - 12 {
            break;
        }
        let active = *app == shell.current_app;
        if active {
            fb.round_rect(ix - 3, iy - 3, icon + 6, icon + 6, 14, Color::rgb(0xff, 0xc7, 0x4c));
        }
        draw_app_icon_modern(fb, *app, ix, iy, icon);
        let lbl = app.label();
        let lw = Framebuffer::measure_text_aa(lbl, FontSize::Px16, false);
        fb.draw_text_aa(
            ix + (icon.saturating_sub(lw)) / 2,
            iy + icon + 4,
            lbl,
            Color::WHITE,
            FontSize::Px16,
            false,
        );
    }

    // Pastida search
    let s_y = y + h - 50;
    fb.round_rect(x + 24, s_y, w - 48, 36, 18, Color::rgb(0x18, 0x1f, 0x36));
    fb.draw_text_aa(x + 40, s_y + 9, "Search apps...", Color::MUTED, FontSize::Px16, false);
}

fn draw_in_app(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // App content area
    fb.round_rect(x + 4, y + 4, w - 8, h - 8, 14, Color::rgb(0x08, 0x12, 0x22));
    shell.draw_app_into(fb, x + 4, y + 4, w - 8, h - 8);
}

fn draw_in_app_nav(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32) {
    let nav_h = 50u32;
    fb.fill_rect(x, y, w, nav_h, Color::rgb(0x10, 0x16, 0x28));

    // 3 ta tugma: Back, Home, Recent (Android navigation)
    let btn_w = w / 3;

    // Back (uchburchak)
    let bx = btn_w / 2 + x;
    let by = y + nav_h / 2;
    fb.fill_rect(bx as u32 - 8, by - 1, 16, 2, Color::WHITE);
    fb.fill_rect(bx as u32 - 8, by - 5, 4, 4, Color::WHITE);
    fb.fill_rect(bx as u32 - 8, by + 1, 4, 4, Color::WHITE);

    // Home (yumaloq)
    let hx = btn_w + btn_w / 2 + x;
    fb.fill_circle(hx as i32, by as i32, 12, Color::WHITE);
    fb.fill_circle(hx as i32, by as i32, 10, Color::rgb(0x10, 0x16, 0x28));

    // Recent (kvadrat)
    let rx = 2 * btn_w + btn_w / 2 + x;
    fb.round_rect(rx as u32 - 9, by - 9, 18, 18, 3, Color::WHITE);
    fb.round_rect(rx as u32 - 7, by - 7, 14, 14, 2, Color::rgb(0x10, 0x16, 0x28));

    let _ = shell;
}

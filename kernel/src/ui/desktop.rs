//! Windows 11 / Linux uslubli desktop layout.
//!
//! - Wallpaper (gradient + accent shapes)
//! - Desktop widgetlari (clock, system, weather)
//! - Pastda taskbar (markazlashgan, Windows 11 uslubli)
//! - Start tugmasi -> Start menu (Tab cikl, Enter open)
//! - Window: title bar (X yopish, kichraytirish, kattalashtirish), content

use alloc::format;

use crate::graphics::framebuffer::{Color, FontSize, Framebuffer};
use crate::ui::apps::App;
use crate::ui::{draw_app_icon_modern, draw_battery_icon, draw_wifi_icon, DesktopView, Shell};

const TASKBAR_H: u32 = 48;

pub fn draw(shell: &Shell, fb: &mut Framebuffer) {
    let w = fb.width();
    let h = fb.height();

    // ---- Wallpaper ----
    draw_wallpaper(fb, w, h);

    // ---- Desktop area ----
    let desktop_h = h - TASKBAR_H;

    // Window (joriy app) — Windows uslubli
    draw_app_window(shell, fb, 32, 24, w - 64, desktop_h - 48);

    // O'ng tomondagi widgetlar (Windows 11 widget panel)
    draw_widgets(shell, fb, w, desktop_h);

    // ---- Taskbar (pastda) ----
    draw_taskbar(shell, fb, w, h);

    // ---- Start menu (agar ochiq) ----
    if shell.desktop_view == DesktopView::StartMenu {
        draw_start_menu(shell, fb, w, h);
    }
}

fn draw_wallpaper(fb: &mut Framebuffer, w: u32, h: u32) {
    // Windows 11 uslubli — ko'k/binafsha gradient
    fb.vgradient(0, 0, w, h, Color::rgb(0x14, 0x1a, 0x3a), Color::rgb(0x06, 0x0a, 0x18));

    // Accent diagonal gradient (rasm sifatida)
    // Burchakdan yorqinroq spot
    for j in 0..h {
        for i in (0..w).step_by(3) {
            let cx = (w as i32) * 3 / 4;
            let cy = (h as i32) / 4;
            let dx = i as i32 - cx;
            let dy = j as i32 - cy;
            let d2 = dx * dx + dy * dy;
            if d2 < 250 * 250 {
                let intensity = 250 * 250 - d2;
                let alpha = (intensity / (250 * 250 / 30)).min(30) as u32;
                if alpha > 0 {
                    let idx = (j * w + i) as usize;
                    let p = fb.raw_pixel(idx);
                    let r = ((p >> 16) & 0xff).min(255 - alpha) + alpha;
                    let g = ((p >> 8) & 0xff).min(255 - alpha) + alpha / 2;
                    let b = (p & 0xff).min(255 - alpha) + alpha;
                    fb.set_raw_pixel(idx, (r << 16) | (g << 8) | b);
                }
            }
        }
    }
}

fn draw_app_window(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // Soya
    fb.shadow_rect(x, y + 6, w, h, Color::rgb(0x00, 0x00, 0x06));

    // Tana
    fb.round_rect(x, y, w, h, 12, Color::rgb(0x1a, 0x1f, 0x2e));

    // Title bar
    let title_h = 36u32;
    fb.round_rect(x, y, w, title_h, 12, Color::rgb(0x24, 0x29, 0x38));
    // Pastki yarmi flat
    fb.fill_rect(x, y + title_h - 6, w, 6, Color::rgb(0x24, 0x29, 0x38));
    fb.hline(x, y + title_h, w, Color::rgb(0x35, 0x3b, 0x4e));

    // App icon (kichik)
    draw_app_icon_modern(fb, shell.current_app, x + 10, y + 6, 24);

    // Window title (AA)
    let title = shell.current_app.label();
    fb.draw_text_aa(x + 44, y + 8, title, Color::WHITE, FontSize::Px20, true);

    // Window controls (Windows 11 — yumaloq, _ □ ×)
    let ctl_size = 30u32;
    let mut cx = x + w - ctl_size;
    let cy = y;
    // Close (X) — qizil hover, oddiy holatda kulrang
    fb.fill_rect(cx, cy, ctl_size, title_h, Color::rgb(0xc4, 0x2b, 0x1c));
    draw_x_glyph(fb, cx + ctl_size / 2 - 6, cy + title_h / 2 - 6, 12, Color::WHITE);
    cx -= ctl_size;
    // Maximize (□)
    draw_square_glyph(fb, cx + ctl_size / 2 - 5, cy + title_h / 2 - 5, 10, Color::WHITE);
    cx -= ctl_size;
    // Minimize (_)
    fb.fill_rect(cx + ctl_size / 2 - 5, cy + title_h / 2 + 4, 10, 2, Color::WHITE);

    // Content area
    let cx = x + 1;
    let cy = y + title_h + 1;
    let cw = w - 2;
    let ch = h - title_h - 2;
    shell.draw_app_into(fb, cx, cy, cw, ch);
}

fn draw_x_glyph(fb: &mut Framebuffer, x: u32, y: u32, size: u32, c: Color) {
    for i in 0..size {
        let px = x + i;
        let py1 = y + i;
        let py2 = y + (size - 1 - i);
        if px < fb.width() && py1 < fb.height() {
            fb.put(px, py1, c);
        }
        if px < fb.width() && py2 < fb.height() {
            fb.put(px, py2, c);
        }
    }
}

fn draw_square_glyph(fb: &mut Framebuffer, x: u32, y: u32, size: u32, c: Color) {
    fb.hline(x, y, size, c);
    fb.hline(x, y + size - 1, size, c);
    fb.vline(x, y, size, c);
    fb.vline(x + size - 1, y, size, c);
}

fn draw_widgets(shell: &Shell, fb: &mut Framebuffer, w: u32, _desktop_h: u32) {
    // Hozircha bitta — desktop wallpaper'da o'ng pastki burchakda mini-widget panel
    // Lekin window joyni egallaydi, shuning uchun widgetlarni window'dan oldin
    // window orqasidagi maydonlarda ko'rsatamiz.
    //
    // Aslida widgetlarni shaffof window orqasida ko'rsatish murakkab.
    // Bu yerda faqat o'ng yuqori burchakda kichik clock chiqaramiz.
    let _ = (shell, fb, w);
}

fn draw_taskbar(shell: &Shell, fb: &mut Framebuffer, w: u32, h: u32) {
    let y = h - TASKBAR_H;

    // Taskbar fon (acrylic blur taqlid — to'q gradient)
    fb.fill_rect(0, y, w, TASKBAR_H, Color::rgb(0x12, 0x16, 0x24));
    fb.hline(0, y, w, Color::rgb(0x32, 0x38, 0x4a));

    // Markazlashgan dock — Start + 8 ta app
    let apps = App::all();
    let visible_count: u32 = 8 + 1; // Start + 8 apps
    let icon = 36u32;
    let gap = 6u32;
    let total_w = visible_count * icon + (visible_count - 1) * gap;
    let start_x = (w - total_w) / 2;

    let mut ix = start_x;
    let iy = y + (TASKBAR_H - icon) / 2;

    // Start tugmasi (Windows logo o'rniga ZaminOS Z)
    let start_active = shell.desktop_view == DesktopView::StartMenu;
    let start_bg = if start_active {
        Color::rgb(0xff, 0xc7, 0x4c)
    } else {
        Color::rgb(0x1f, 0x29, 0x44)
    };
    let start_fg = if start_active { Color::ZAMIN_BG } else { Color::WHITE };
    fb.round_rect(ix, iy, icon, icon, 6, start_bg);
    fb.draw_text_aa_centered(ix + icon / 2, iy + 6, "Z", start_fg, FontSize::Px24, true);
    if start_active {
        fb.fill_rect(ix + 6, iy + icon - 3, icon - 12, 2, Color::ZAMIN_FG);
    }
    ix += icon + gap;

    // App ikonalari (8 ta — Settings dock'da yo'q)
    for app in apps.iter().take(8) {
        let active = *app == shell.current_app && shell.desktop_view == DesktopView::Desktop;
        // Hover/active background
        if active {
            fb.round_rect(ix - 2, iy - 2, icon + 4, icon + 4, 8, Color::rgb(0x2a, 0x32, 0x4a));
        }
        draw_app_icon_modern(fb, *app, ix, iy, icon);
        // Active indicator (chiziqcha pastida)
        if active {
            fb.fill_rect(ix + icon / 2 - 6, iy + icon + 2, 12, 2, Color::ACCENT);
        }
        ix += icon + gap;
    }

    // O'ng tomon: system tray
    let mut sx = w - 12;
    let clock_text = format!("{:02}:{:02}", shell.uptime_ticks / 60, shell.uptime_ticks % 60);
    let clock_w = Framebuffer::measure_text_aa(&clock_text, FontSize::Px16, true);
    sx = sx.saturating_sub(clock_w);
    fb.draw_text_aa(sx, y + 8, &clock_text, Color::WHITE, FontSize::Px16, true);
    let date_text = format!("{} sek", shell.uptime_ticks);
    let date_w = Framebuffer::measure_text_aa(&date_text, FontSize::Px16, false);
    sx = sx.saturating_sub(date_w);
    fb.draw_text_aa(sx, y + 26, &date_text, Color::MUTED, FontSize::Px16, false);

    sx = sx.saturating_sub(40);
    draw_battery_icon(fb, sx, y + (TASKBAR_H - 10) / 2);
    sx = sx.saturating_sub(24);
    draw_wifi_icon(fb, sx, y + (TASKBAR_H - 10) / 2);
}

fn draw_start_menu(shell: &Shell, fb: &mut Framebuffer, w: u32, h: u32) {
    // Start menu: pastdan markazda ko'tariladi (Windows 11 uslubi)
    let menu_w = 540u32;
    let menu_h = 460u32;
    let menu_x = (w - menu_w) / 2;
    let menu_y = h - TASKBAR_H - menu_h - 12;

    // Soya
    fb.shadow_rect(menu_x, menu_y + 8, menu_w, menu_h, Color::rgb(0x00, 0x00, 0x06));

    // Asosiy panel — yarim shaffof acrylic taqlid
    fb.round_rect(menu_x, menu_y, menu_w, menu_h, 16, Color::rgb(0x18, 0x1d, 0x2e));

    // Search bar (yuqorida)
    let pad = 24u32;
    let search_y = menu_y + 24;
    fb.round_rect(menu_x + pad, search_y, menu_w - 2 * pad, 36, 8, Color::rgb(0x10, 0x14, 0x22));
    fb.draw_text_aa(menu_x + pad + 16, search_y + 8, "Search apps, files...", Color::MUTED, FontSize::Px20, false);

    // "Pinned" header
    let header_y = search_y + 56;
    fb.draw_text_aa(menu_x + pad, header_y, "Pinned", Color::WHITE, FontSize::Px20, true);

    // App grid (4x2)
    let grid_y = header_y + 32;
    let cols = 4;
    let rows = 2;
    let icon = 56u32;
    let h_gap = 28u32;
    let v_gap = 28u32;
    let row_total_w = cols * icon + (cols - 1) * h_gap;
    let grid_x = menu_x + (menu_w - row_total_w) / 2;

    let apps = App::all();
    for (idx, app) in apps.iter().enumerate() {
        let r = (idx as u32) / cols;
        let c = (idx as u32) % cols;
        if r >= rows {
            break;
        }
        let ix = grid_x + c * (icon + h_gap);
        let iy = grid_y + r * (icon + 24 + v_gap);
        let active = *app == shell.current_app;
        if active {
            fb.round_rect(ix - 4, iy - 4, icon + 8, icon + 8, 12, Color::rgb(0x2a, 0x35, 0x55));
        }
        draw_app_icon_modern(fb, *app, ix, iy, icon);
        // Label
        let label = app.label();
        let lw = Framebuffer::measure_text_aa(label, FontSize::Px16, false);
        fb.draw_text_aa(
            ix + (icon.saturating_sub(lw)) / 2,
            iy + icon + 4,
            label,
            Color::WHITE,
            FontSize::Px16,
            false,
        );
    }

    // Pastida — user info
    let user_y = menu_y + menu_h - 50;
    fb.fill_circle((menu_x + pad + 16) as i32, (user_y + 16) as i32, 14, Color::rgb(0xff, 0xc7, 0x4c));
    fb.draw_text_aa(menu_x + pad + 40, user_y + 8, "ZaminOS user", Color::WHITE, FontSize::Px16, true);

    // Power tugmasi (o'ng pastda)
    let pwr_x = menu_x + menu_w - pad - 28;
    let pwr_y = user_y + 4;
    fb.fill_circle((pwr_x + 14) as i32, (pwr_y + 14) as i32, 14, Color::rgb(0x35, 0x3b, 0x52));
    fb.draw_text_aa_centered(pwr_x + 14, pwr_y + 6, "U", Color::ACCENT, FontSize::Px16, true);
}

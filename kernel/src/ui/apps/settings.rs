//! Settings app — toggles va sozlamalar (zamonaviy iOS uslubli).

use alloc::format;

use crate::graphics::framebuffer::{Color, FontSize, Framebuffer};
use crate::ui::Shell;

pub struct SettingsState {
    pub dark_mode: bool,
    pub wifi: bool,
    pub bluetooth: bool,
    pub do_not_disturb: bool,
    pub volume: u32,
    pub brightness: u32,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            dark_mode: true, wifi: true, bluetooth: false, do_not_disturb: false,
            volume: 70, brightness: 80,
        }
    }
    pub fn handle_key(&mut self, code: u16) -> bool {
        match code {
            2 => { self.dark_mode = !self.dark_mode; true }
            3 => { self.wifi = !self.wifi; true }
            4 => { self.bluetooth = !self.bluetooth; true }
            5 => { self.do_not_disturb = !self.do_not_disturb; true }
            12 => { self.volume = self.volume.saturating_sub(10); true }
            13 => { self.volume = (self.volume + 10).min(100); true }
            26 => { self.brightness = self.brightness.saturating_sub(10); true }
            27 => { self.brightness = (self.brightness + 10).min(100); true }
            _ => false,
        }
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, _h: u32) {
    fb.draw_text_aa(x + 20, y + 14, "Settings", Color::ZAMIN_FG, FontSize::Px32, true);

    fb.draw_text_aa(x + 20, y + 60, "System", Color::MUTED, FontSize::Px16, true);

    let row_y = y + 84;
    let row_h = 44u32;
    let toggles = [
        ("Dark Mode",      "[1]", shell.settings.dark_mode),
        ("Wi-Fi",          "[2]", shell.settings.wifi),
        ("Bluetooth",      "[3]", shell.settings.bluetooth),
        ("Do Not Disturb", "[4]", shell.settings.do_not_disturb),
    ];
    let pad = 16u32;
    let mut ry = row_y;
    for (label, key, on) in toggles.iter() {
        fb.round_rect(x + pad, ry, w - 2 * pad, row_h - 6, 8, Color::rgb(0x12, 0x1c, 0x32));
        fb.draw_text_aa(x + pad + 14, ry + 8, label, Color::WHITE, FontSize::Px20, false);
        fb.draw_text_aa(x + pad + 14, ry + 26, key, Color::MUTED, FontSize::Px16, false);

        // Toggle switch
        let sw_w = 56u32;
        let sw_h = 28u32;
        let sw_x = x + w - pad - 16 - sw_w;
        let sw_y = ry + (row_h - 6 - sw_h) / 2;
        let bg = if *on { Color::rgb(0x4c, 0xaf, 0x50) } else { Color::rgb(0x40, 0x48, 0x5a) };
        fb.round_rect(sw_x, sw_y, sw_w, sw_h, sw_h / 2, bg);
        let knob_x = if *on { sw_x + sw_w - sw_h - 1 } else { sw_x + 1 };
        fb.fill_circle(
            (knob_x + sw_h / 2) as i32,
            (sw_y + sw_h / 2) as i32,
            sw_h / 2 - 3,
            Color::WHITE,
        );

        ry += row_h;
    }

    // Sliders
    ry += 8;
    fb.draw_text_aa(x + pad + 14, ry, "Volume [-/+]", Color::WHITE, FontSize::Px20, false);
    fb.draw_text_aa(x + w - pad - 70, ry, &format!("{}%", shell.settings.volume), Color::ACCENT, FontSize::Px20, true);
    draw_slider(fb, x + pad + 14, ry + 32, w - 2 * pad - 28, shell.settings.volume, Color::rgb(0xec, 0x40, 0x7a));
    ry += 64;

    fb.draw_text_aa(x + pad + 14, ry, "Brightness [/]", Color::WHITE, FontSize::Px20, false);
    fb.draw_text_aa(x + w - pad - 70, ry, &format!("{}%", shell.settings.brightness), Color::ACCENT, FontSize::Px20, true);
    draw_slider(fb, x + pad + 14, ry + 32, w - 2 * pad - 28, shell.settings.brightness, Color::rgb(0xff, 0xc7, 0x4c));
}

fn draw_slider(fb: &mut Framebuffer, x: u32, y: u32, w: u32, value: u32, fill: Color) {
    fb.round_rect(x, y, w, 10, 5, Color::rgb(0x30, 0x36, 0x4a));
    let fw = (w * value) / 100;
    if fw > 0 {
        fb.round_rect(x, y, fw, 10, 5, fill);
    }
    let knob_x = x + fw;
    fb.fill_circle(knob_x as i32, (y + 5) as i32, 9, Color::WHITE);
    fb.fill_circle(knob_x as i32, (y + 5) as i32, 6, fill);
}

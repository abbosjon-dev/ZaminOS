//! Settings app — toggles va sozlamalar.

use alloc::format;

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

pub struct SettingsState {
    pub dark_mode: bool,
    pub wifi: bool,
    pub bluetooth: bool,
    pub do_not_disturb: bool,
    pub volume: u32,    // 0..100
    pub brightness: u32,
}

impl SettingsState {
    pub fn new() -> Self {
        Self {
            dark_mode: true,
            wifi: true,
            bluetooth: false,
            do_not_disturb: false,
            volume: 70,
            brightness: 80,
        }
    }
    pub fn handle_key(&mut self, code: u16) -> bool {
        // 1..6 toggles
        match code {
            2 => { self.dark_mode = !self.dark_mode; true }
            3 => { self.wifi = !self.wifi; true }
            4 => { self.bluetooth = !self.bluetooth; true }
            5 => { self.do_not_disturb = !self.do_not_disturb; true }
            // -/+ for volume
            12 => { self.volume = self.volume.saturating_sub(10); true }
            13 => { self.volume = (self.volume + 10).min(100); true }
            // [/] for brightness
            26 => { self.brightness = self.brightness.saturating_sub(10); true }
            27 => { self.brightness = (self.brightness + 10).min(100); true }
            _ => false,
        }
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_rect(x, y, w, 22, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 8, y + 6, "Settings", Color::ZAMIN_FG, 1);

    fb.draw_text(x + 16, y + 36, "System", Color::ZAMIN_FG, 3);

    let mut row_y = y + 90;
    let row_h = 36u32;
    let toggles = [
        ("[1] Dark Mode",       shell.settings.dark_mode),
        ("[2] Wi-Fi",           shell.settings.wifi),
        ("[3] Bluetooth",       shell.settings.bluetooth),
        ("[4] Do Not Disturb",  shell.settings.do_not_disturb),
    ];
    for (label, on) in toggles.iter() {
        // Row background (alternating)
        fb.round_rect(x + 16, row_y, w - 32, row_h - 4, 6, Color::rgb(0x12, 0x1c, 0x32));
        fb.draw_text(x + 26, row_y + 8, label, Color::WHITE, 2);

        // Toggle switch
        let sw_w = 50u32;
        let sw_h = 22u32;
        let sw_x = x + w - 32 - sw_w;
        let sw_y = row_y + (row_h - 4 - sw_h) / 2;
        let bg = if *on {
            Color::rgb(0x4c, 0xaf, 0x50)
        } else {
            Color::rgb(0x40, 0x48, 0x5a)
        };
        fb.round_rect(sw_x, sw_y, sw_w, sw_h, sw_h / 2, bg);
        let knob_x = if *on { sw_x + sw_w - sw_h - 1 } else { sw_x + 1 };
        fb.fill_circle(
            (knob_x + sw_h / 2) as i32,
            (sw_y + sw_h / 2) as i32,
            sw_h / 2 - 2,
            Color::WHITE,
        );

        row_y += row_h + 2;
    }

    // Sliders
    row_y += 10;
    fb.draw_text(x + 16, row_y, "[-/+] Volume", Color::WHITE, 1);
    draw_slider(fb, x + 16, row_y + 16, w - 32, shell.settings.volume, Color::rgb(0xec, 0x40, 0x7a));
    fb.draw_text(x + w - 50, row_y, &format!("{}%", shell.settings.volume), Color::ACCENT, 1);
    row_y += 36;

    fb.draw_text(x + 16, row_y, "[ [/]  ] Brightness", Color::WHITE, 1);
    draw_slider(fb, x + 16, row_y + 16, w - 32, shell.settings.brightness, Color::rgb(0xff, 0xc7, 0x4c));
    fb.draw_text(x + w - 50, row_y, &format!("{}%", shell.settings.brightness), Color::ACCENT, 1);

    // Hint
    let st_y = y + h - 14;
    fb.fill_rect(x, st_y, w, 14, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 6, st_y + 2,
        "1-4 toggle, -/+ volume, [/] brightness",
        Color::MUTED, 1);
}

fn draw_slider(fb: &mut Framebuffer, x: u32, y: u32, w: u32, value: u32, fill: Color) {
    fb.round_rect(x, y, w, 8, 4, Color::rgb(0x30, 0x36, 0x4a));
    let fw = (w * value) / 100;
    if fw > 0 {
        fb.round_rect(x, y, fw, 8, 4, fill);
    }
    // Knob
    let knob_x = x + fw;
    fb.fill_circle(knob_x as i32, (y + 4) as i32, 6, Color::WHITE);
}

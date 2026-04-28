//! Music app — soxta media player.

use alloc::format;

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

pub struct MusicState {
    pub playing: bool,
    pub track: usize,
    pub progress: u32,  // 0..100
}

const TRACKS: &[(&str, &str, u32)] = &[
    ("ZaminOS Boot Theme",  "ZaminOS",     224),
    ("Terminal Lullaby",    "kernel_main", 187),
    ("aarch64 Anthem",      "ARM Ltd.",    302),
    ("Mobile Convergence",  "Faza 7",      256),
];

impl MusicState {
    pub fn new() -> Self {
        Self { playing: true, track: 0, progress: 27 }
    }
    pub fn handle_key(&mut self, code: u16) -> bool {
        // Space = play/pause
        if code == 57 {
            self.playing = !self.playing;
            return true;
        }
        // 'k' next, 'j' prev
        if code == 37 {
            if self.track + 1 < TRACKS.len() { self.track += 1; }
            self.progress = 0;
            return true;
        }
        if code == 36 {
            if self.track > 0 { self.track -= 1; }
            self.progress = 0;
            return true;
        }
        false
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // Title
    fb.fill_rect(x, y, w, 22, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 8, y + 6, "Music", Color::ZAMIN_FG, 1);

    let track = TRACKS[shell.music.track.min(TRACKS.len() - 1)];

    // Album art (gradient kvadrat)
    let art_size = (h - 22).min(w / 3).min(220);
    let art_x = x + 30;
    let art_y = y + 50;
    fb.shadow_rect(art_x, art_y + 4, art_size, art_size, Color::rgb(0x00, 0x00, 0x06));
    fb.round_rect(art_x, art_y, art_size, art_size, 14, Color::rgb(0xec, 0x40, 0x7a));
    fb.vgradient(art_x + 4, art_y + art_size / 3, art_size - 8, art_size * 2 / 3 - 4,
        Color::rgb(0xec, 0x40, 0x7a), Color::rgb(0x6a, 0x1b, 0x9a));
    // Music note glyph (cataclysmic 8x8 expanded to 5)
    let note: &[u8; 8] = &[
        0b00011110,
        0b00010010,
        0b00010010,
        0b00010010,
        0b00010010,
        0b01110010,
        0b11110010,
        0b11100000,
    ];
    let note_scale = (art_size / 16).max(2);
    let note_w = 8 * note_scale;
    let note_x = art_x + (art_size - note_w) / 2;
    let note_y = art_y + (art_size - note_w) / 2;
    for (row, byte) in note.iter().enumerate() {
        for col in 0..8u32 {
            if byte & (1 << (7 - col)) != 0 {
                fb.fill_rect(
                    note_x + col * note_scale,
                    note_y + row as u32 * note_scale,
                    note_scale, note_scale,
                    Color::WHITE,
                );
            }
        }
    }

    // Track info
    let info_x = art_x + art_size + 24;
    let info_w = (x + w).saturating_sub(info_x).saturating_sub(20);
    fb.draw_text(info_x, art_y + 16, "Now Playing", Color::ACCENT, 1);
    fb.draw_text(info_x, art_y + 36, track.0, Color::WHITE, 2);
    fb.draw_text(info_x, art_y + 60, track.1, Color::MUTED, 1);

    // Progress bar
    let bar_y = art_y + 110;
    let bar_w = info_w;
    fb.round_rect(info_x, bar_y, bar_w, 6, 3, Color::rgb(0x30, 0x36, 0x4a));
    let prog_w = (bar_w * shell.music.progress) / 100;
    if prog_w > 0 {
        fb.round_rect(info_x, bar_y, prog_w, 6, 3, Color::rgb(0xec, 0x40, 0x7a));
    }
    let cur_min = (track.2 * shell.music.progress / 100) / 60;
    let cur_sec = (track.2 * shell.music.progress / 100) % 60;
    let tot_min = track.2 / 60;
    let tot_sec = track.2 % 60;
    fb.draw_text(info_x, bar_y + 12, &format!("{:02}:{:02}", cur_min, cur_sec), Color::MUTED, 1);
    fb.draw_text(info_x + bar_w - 40, bar_y + 12, &format!("{:02}:{:02}", tot_min, tot_sec), Color::MUTED, 1);

    // Controls
    let ctrl_y = bar_y + 38;
    let ctrl_size = 32u32;
    let ctrl_gap = 14u32;
    let ctrl_total = 3 * ctrl_size + 2 * ctrl_gap;
    let ctrl_start = info_x + (info_w.saturating_sub(ctrl_total)) / 2;

    // Prev
    draw_circle_button(fb, ctrl_start, ctrl_y, ctrl_size, "<<");
    // Play/Pause (kattaroq)
    let mid_x = ctrl_start + ctrl_size + ctrl_gap;
    fb.shadow_rect(mid_x, ctrl_y + 2, ctrl_size, ctrl_size, Color::rgb(0x00, 0x00, 0x06));
    fb.round_rect(mid_x, ctrl_y, ctrl_size, ctrl_size, ctrl_size / 2, Color::ZAMIN_FG);
    let glyph = if shell.music.playing { "||" } else { "▶" };
    let g_text = if shell.music.playing { "II" } else { ">" };
    let _ = glyph;
    fb.draw_text(mid_x + (ctrl_size - 16) / 2, ctrl_y + (ctrl_size - 16) / 2, g_text, Color::ZAMIN_BG, 2);
    // Next
    draw_circle_button(fb, mid_x + ctrl_size + ctrl_gap, ctrl_y, ctrl_size, ">>");

    // Track list
    let list_y = ctrl_y + ctrl_size + 24;
    fb.draw_text(info_x, list_y, "Tracklar:", Color::ACCENT, 1);
    let mut ly = list_y + 18;
    for (i, t) in TRACKS.iter().enumerate() {
        let active = i == shell.music.track;
        let fg = if active { Color::ZAMIN_FG } else { Color::MUTED };
        fb.draw_text(info_x, ly, &format!("{}. {}", i + 1, t.0), fg, 1);
        ly += 16;
    }

    // Hint
    let st_y = y + h - 14;
    fb.fill_rect(x, st_y, w, 14, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 6, st_y + 2,
        "Space = play/pause, j/k = oldingi/keyingi", Color::MUTED, 1);
}

fn draw_circle_button(fb: &mut Framebuffer, x: u32, y: u32, size: u32, label: &str) {
    fb.shadow_rect(x, y + 2, size, size, Color::rgb(0x00, 0x00, 0x06));
    fb.round_rect(x, y, size, size, size / 2, Color::rgb(0x33, 0x3d, 0x52));
    let lw = (label.len() as u32) * 8;
    fb.draw_text(x + (size.saturating_sub(lw)) / 2, y + (size - 8) / 2, label, Color::WHITE, 1);
}

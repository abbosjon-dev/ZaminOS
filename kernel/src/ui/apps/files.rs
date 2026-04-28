//! Files app — soxta fayl tizimi browser.

use crate::graphics::framebuffer::{Color, FontSize, Framebuffer};
use crate::ui::Shell;

#[derive(Clone, Copy)]
pub struct FileEntry {
    pub name: &'static str,
    pub kind: FileKind,
    pub size: &'static str,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FileKind { Folder, Doc, Image, Audio, Code }

const FILES: &[FileEntry] = &[
    FileEntry { name: "Documents", kind: FileKind::Folder, size: "" },
    FileEntry { name: "Pictures",  kind: FileKind::Folder, size: "" },
    FileEntry { name: "Music",     kind: FileKind::Folder, size: "" },
    FileEntry { name: "Code",      kind: FileKind::Folder, size: "" },
    FileEntry { name: "README.md",     kind: FileKind::Doc,   size: "2.4 KB" },
    FileEntry { name: "boot.S",        kind: FileKind::Code,  size: "1.8 KB" },
    FileEntry { name: "kernel.elf",    kind: FileKind::Code,  size: "162 KB" },
    FileEntry { name: "wallpaper.png", kind: FileKind::Image, size: "412 KB" },
    FileEntry { name: "song.mp3",      kind: FileKind::Audio, size: "3.7 MB" },
    FileEntry { name: "notes.txt",     kind: FileKind::Doc,   size: "0.8 KB" },
];

pub struct FilesState { pub selected: usize }
impl FilesState {
    pub fn new() -> Self { Self { selected: 0 } }
    pub fn handle_key(&mut self, code: u16) -> bool {
        if code == 36 { if self.selected + 1 < FILES.len() { self.selected += 1; } return true; }
        if code == 37 { if self.selected > 0 { self.selected -= 1; } return true; }
        false
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // Sidebar
    let sb_w = 130u32;
    fb.fill_rect(x, y, sb_w, h, Color::rgb(0x10, 0x18, 0x2a));
    fb.draw_text_aa(x + 14, y + 16, "Locations", Color::ACCENT, FontSize::Px16, true);
    let sidebar = ["Home", "Apps", "Disk", "Trash", "Network"];
    let mut sy = y + 44;
    for (i, item) in sidebar.iter().enumerate() {
        let active = i == 0;
        if active {
            fb.round_rect(x + 8, sy - 4, sb_w - 16, 28, 6, Color::rgb(0x1c, 0x65, 0xc0));
        }
        fb.draw_text_aa(x + 18, sy, item, Color::WHITE, FontSize::Px16, active);
        sy += 32;
    }

    // Header
    let lx = x + sb_w + 16;
    let lw = w.saturating_sub(sb_w + 32);
    fb.draw_text_aa(lx, y + 12, "~/zamin", Color::WHITE, FontSize::Px24, true);
    fb.draw_text_aa(lx, y + 46, "Name", Color::ACCENT, FontSize::Px16, true);
    fb.draw_text_aa(lx + lw - 80, y + 46, "Size", Color::ACCENT, FontSize::Px16, true);
    fb.hline(lx, y + 70, lw, Color::rgb(0x30, 0x36, 0x4a));

    let mut ly = y + 80;
    for (i, file) in FILES.iter().enumerate() {
        let selected = i == shell.files.selected;
        if selected {
            fb.round_rect(lx - 6, ly - 4, lw, 28, 6, Color::rgb(0x1c, 0x65, 0xc0));
        }
        draw_file_icon(fb, lx, ly + 2, file.kind);
        fb.draw_text_aa(lx + 28, ly, file.name, Color::WHITE, FontSize::Px16, selected);
        if !file.size.is_empty() {
            fb.draw_text_aa(lx + lw - 80, ly, file.size, Color::MUTED, FontSize::Px16, false);
        } else {
            fb.draw_text_aa(lx + lw - 80, ly, "—", Color::MUTED, FontSize::Px16, false);
        }
        ly += 30;
    }

    // Status bar
    let st_y = y + h - 18;
    fb.fill_rect(x, st_y, w, 18, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text_aa(x + 8, st_y + 2,
        "j/k navigatsiya, Enter ochish",
        Color::MUTED, FontSize::Px16, false);
}

fn draw_file_icon(fb: &mut Framebuffer, x: u32, y: u32, kind: FileKind) {
    let (top, bot) = match kind {
        FileKind::Folder => (Color::rgb(0xff, 0xc1, 0x07), Color::rgb(0xf5, 0x7c, 0x00)),
        FileKind::Doc => (Color::rgb(0x90, 0xa4, 0xae), Color::rgb(0x60, 0x7d, 0x8b)),
        FileKind::Image => (Color::rgb(0x66, 0xbb, 0x6a), Color::rgb(0x2e, 0x7d, 0x32)),
        FileKind::Audio => (Color::rgb(0xec, 0x40, 0x7a), Color::rgb(0xad, 0x14, 0x57)),
        FileKind::Code => (Color::rgb(0x42, 0xa5, 0xf5), Color::rgb(0x15, 0x65, 0xc0)),
    };
    fb.round_rect(x, y, 18, 18, 4, top);
    fb.vgradient(x + 1, y + 9, 16, 9, top, bot);
}

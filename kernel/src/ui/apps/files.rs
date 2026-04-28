//! Files app — soxta fayl tizimi browser.

use crate::graphics::framebuffer::{Color, Framebuffer};
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
    FileEntry { name: "README.md",       kind: FileKind::Doc,   size: "2.4 KB" },
    FileEntry { name: "boot.S",          kind: FileKind::Code,  size: "1.8 KB" },
    FileEntry { name: "kernel.elf",      kind: FileKind::Code,  size: "162 KB" },
    FileEntry { name: "wallpaper.png",   kind: FileKind::Image, size: "412 KB" },
    FileEntry { name: "song.mp3",        kind: FileKind::Audio, size: "3.7 MB" },
    FileEntry { name: "notes.txt",       kind: FileKind::Doc,   size: "0.8 KB" },
];

pub struct FilesState {
    pub selected: usize,
}

impl FilesState {
    pub fn new() -> Self {
        Self { selected: 0 }
    }
    pub fn handle_key(&mut self, code: u16) -> bool {
        // 'j' = down, 'k' = up
        if code == 36 {
            if self.selected + 1 < FILES.len() {
                self.selected += 1;
            }
            return true;
        }
        if code == 37 {
            if self.selected > 0 {
                self.selected -= 1;
            }
            return true;
        }
        false
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    fb.fill_rect(x, y, w, 22, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 8, y + 6, "Files  -  ~/zamin", Color::ZAMIN_FG, 1);

    // Sidebar
    let sb_w = 110u32;
    fb.fill_rect(x, y + 22, sb_w, h - 22, Color::rgb(0x10, 0x18, 0x2a));
    let sidebar = ["Home", "Apps", "Disk", "Trash", "Network"];
    let mut sy = y + 34;
    for (i, item) in sidebar.iter().enumerate() {
        let active = i == 0;
        if active {
            fb.fill_rect(x + 4, sy - 2, sb_w - 8, 18, Color::ZAMIN_FG);
            fb.draw_text(x + 12, sy + 2, item, Color::ZAMIN_BG, 1);
        } else {
            fb.draw_text(x + 12, sy + 2, item, Color::WHITE, 1);
        }
        sy += 22;
    }

    // Header
    let lx = x + sb_w + 8;
    let lw = w.saturating_sub(sb_w + 16);
    fb.draw_text(lx, y + 30, "Name", Color::ACCENT, 1);
    fb.draw_text(lx + lw - 80, y + 30, "Size", Color::ACCENT, 1);
    fb.hline(lx, y + 44, lw, Color::rgb(0x30, 0x36, 0x4a));

    // List
    let mut ly = y + 50;
    for (i, file) in FILES.iter().enumerate() {
        let selected = i == shell.files.selected;
        if selected {
            fb.round_rect(lx - 2, ly - 2, lw, 22, 4, Color::rgb(0x1c, 0x65, 0xc0));
        }
        // Icon
        draw_file_icon(fb, lx + 2, ly, file.kind);
        let fg = if selected { Color::WHITE } else { Color::WHITE };
        fb.draw_text(lx + 22, ly + 4, file.name, fg, 1);
        if !file.size.is_empty() {
            fb.draw_text(lx + lw - 80, ly + 4, file.size, Color::MUTED, 1);
        } else {
            fb.draw_text(lx + lw - 80, ly + 4, "—", Color::MUTED, 1);
        }
        ly += 22;
    }

    // Status bar
    let st_y = y + h - 14;
    fb.fill_rect(x, st_y, w, 14, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 6, st_y + 2,
        "j/k tugmalari bilan navigatsiya, Enter bosish — ochish",
        Color::MUTED, 1);
}

fn draw_file_icon(fb: &mut Framebuffer, x: u32, y: u32, kind: FileKind) {
    let (top, bot) = match kind {
        FileKind::Folder => (Color::rgb(0xff, 0xc1, 0x07), Color::rgb(0xf5, 0x7c, 0x00)),
        FileKind::Doc => (Color::rgb(0x90, 0xa4, 0xae), Color::rgb(0x60, 0x7d, 0x8b)),
        FileKind::Image => (Color::rgb(0x66, 0xbb, 0x6a), Color::rgb(0x2e, 0x7d, 0x32)),
        FileKind::Audio => (Color::rgb(0xec, 0x40, 0x7a), Color::rgb(0xad, 0x14, 0x57)),
        FileKind::Code => (Color::rgb(0x42, 0xa5, 0xf5), Color::rgb(0x15, 0x65, 0xc0)),
    };
    fb.round_rect(x, y + 1, 16, 16, 3, top);
    fb.vgradient(x + 1, y + 8, 14, 8, top, bot);
    let symbol = match kind {
        FileKind::Folder => "/",
        FileKind::Doc => "T",
        FileKind::Image => "I",
        FileKind::Audio => "M",
        FileKind::Code => "C",
    };
    fb.draw_text(x + 5, y + 5, symbol, Color::WHITE, 1);
}

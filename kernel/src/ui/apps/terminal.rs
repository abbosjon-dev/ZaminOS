//! Terminal app — sodda REPL.

use alloc::collections::VecDeque;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::drivers::input;
use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

const MAX_LINES: usize = 16;

pub struct TerminalState {
    /// Tarixiy chiqish satrlari (eng yangi oxirida).
    history: VecDeque<String>,
    /// Joriy yozilayotgan satr.
    line: String,
}

impl TerminalState {
    pub fn new() -> Self {
        let mut s = Self {
            history: VecDeque::new(),
            line: String::new(),
        };
        s.write("ZaminOS Terminal v0.1");
        s.write("'help' deb yozing va Enter bosing");
        s
    }

    fn write(&mut self, line: &str) {
        if self.history.len() >= MAX_LINES {
            self.history.pop_front();
        }
        self.history.push_back(String::from(line));
    }

    pub fn handle_key(&mut self, code: u16) -> bool {
        // Backspace
        if code == 14 {
            self.line.pop();
            return true;
        }
        // Enter
        if code == 28 {
            let cmd = self.line.clone();
            self.write(&format!("$ {}", cmd));
            self.line.clear();
            self.exec(&cmd);
            return true;
        }
        // Boshqa harflar — qaytaramiz, asosiy handler `typed` ga qo'shadi
        if let Some(ch) = input::keycode_to_char(code) {
            if ch != '\x08' && ch != '\n' {
                if self.line.len() < 60 {
                    self.line.push(ch);
                }
                return true;
            }
        }
        false
    }

    fn exec(&mut self, raw: &str) {
        let cmd = raw.trim();
        if cmd.is_empty() {
            return;
        }
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        match parts[0] {
            "help" => {
                self.write("Mavjud buyruqlar:");
                self.write("  help        - bu xabar");
                self.write("  echo TEXT   - matnni qaytarish");
                self.write("  clear       - ekranni tozalash");
                self.write("  ps          - jarayonlar");
                self.write("  mem         - heap holati");
                self.write("  uname       - tizim haqida");
                self.write("  about       - ZaminOS haqida");
            }
            "echo" => {
                let rest = if parts.len() > 1 { parts[1..].join(" ") } else { String::new() };
                self.write(&rest);
            }
            "clear" => {
                self.history.clear();
            }
            "ps" => {
                self.write("PID  STATE   NAME");
                self.write("  1  running kernel_main");
                self.write("  2  sleeping idle (wfi)");
                self.write("  3  active   shell (UI)");
                self.write("  4  ready    timer-irq");
            }
            "mem" => {
                let stats = crate::memory::heap::stats();
                self.write(&format!("size  : {} KiB", stats.0 / 1024));
                self.write(&format!("used  : {} bytes", stats.1));
                self.write(&format!("free  : {} KiB", stats.2 / 1024));
            }
            "uname" => {
                self.write("ZaminOS v0.7.0 aarch64 (QEMU virt) Rust nightly");
            }
            "about" => {
                self.write("ZaminOS - mobile-first konvergent OS");
                self.write("aarch64 + Rust + ramfb + virtio-input");
            }
            other => {
                self.write(&format!("buyruq topilmadi: {}", other));
                self.write("'help' bilan ro'yxat");
            }
        }
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    let big = w >= 500;
    let scale = if big { 1 } else { 1 };
    let line_h: u32 = 14;

    // Title bar
    fb.fill_rect(x, y, w, 22, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 8, y + 6, "Terminal  >_", Color::ZAMIN_FG, 1);

    // Terminal body
    let body_y = y + 24;
    let body_h = h - 24;
    fb.fill_rect(x, body_y, w, body_h, Color::rgb(0x02, 0x06, 0x10));

    // History chiqishi (pastdan yuqoriga)
    let max_lines = (body_h / line_h).saturating_sub(2) as usize;
    let history = &shell.terminal.history;
    let start = history.len().saturating_sub(max_lines);
    let mut ly = body_y + 4;
    for line in history.iter().skip(start) {
        let color = if line.starts_with("$ ") {
            Color::ZAMIN_FG
        } else {
            Color::WHITE
        };
        fb.draw_text(x + 6, ly, line, color, scale);
        ly += line_h;
    }

    // Joriy prompt
    let prompt_y = body_y + body_h - line_h - 2;
    fb.fill_rect(x, prompt_y, w, line_h + 2, Color::rgb(0x05, 0x0a, 0x16));
    let prompt = format!("$ {}", shell.terminal.line);
    fb.draw_text(x + 6, prompt_y + 2, &prompt, Color::SUCCESS, scale);
    // Caret
    let caret_x = x + 6 + (prompt.len() as u32) * 8;
    fb.fill_rect(caret_x + 1, prompt_y + 2, 2, 10, Color::SUCCESS);
}

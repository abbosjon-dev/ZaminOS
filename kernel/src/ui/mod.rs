//! ZaminOS shell UI — konvergent layout (top bar + side panel + main area).
//!
//! Bu Faza 6.5: single-window shell. Faza 7-9 da to'liq compositor +
//! mobile/desktop adaptiv layout qo'shiladi.

use alloc::format;
use alloc::string::{String, ToString};

use crate::drivers::input;
use crate::graphics::framebuffer::{Color, Framebuffer};

#[derive(Clone, Copy, PartialEq)]
pub enum App {
    Welcome,
    SysMon,
    Keyboard,
}

impl App {
    fn next(self) -> Self {
        match self {
            App::Welcome => App::SysMon,
            App::SysMon => App::Keyboard,
            App::Keyboard => App::Welcome,
        }
    }

    fn label(self) -> &'static str {
        match self {
            App::Welcome => "Welcome",
            App::SysMon => "SysMon",
            App::Keyboard => "Keyboard",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            App::Welcome => "Z", // ZaminOS
            App::SysMon => "S",  // System monitor
            App::Keyboard => "K",
        }
    }
}

pub struct Shell {
    pub current_app: App,
    pub uptime_ticks: u32,
    pub event_count: u32,
    pub typed: String,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub heap_used: usize,
    pub heap_size: usize,
}

impl Shell {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            current_app: App::Welcome,
            uptime_ticks: 0,
            event_count: 0,
            typed: String::new(),
            cursor_x: (width / 2) as i32,
            cursor_y: (height / 2) as i32,
            heap_used: 0,
            heap_size: 0,
        }
    }

    pub fn handle_input(&mut self, ev_type: u16, code: u16, value: u32) -> bool {
        match ev_type {
            x if x == input::EV_KEY => {
                if value != 1 {
                    return false;
                }
                self.event_count += 1;
                // 15 = TAB
                if code == 15 {
                    self.current_app = self.current_app.next();
                    return true;
                }
                if let Some(ch) = input::keycode_to_char(code) {
                    if ch == '\x08' {
                        self.typed.pop();
                    } else if self.typed.len() < 60 {
                        self.typed.push(ch);
                    }
                    return true;
                }
                false
            }
            x if x == input::EV_ABS => {
                self.event_count += 1;
                match code {
                    c if c == input::ABS_X => {
                        self.cursor_x = ((value as u64 * 800) / 32768) as i32;
                        true
                    }
                    c if c == input::ABS_Y => {
                        self.cursor_y = ((value as u64 * 600) / 32768) as i32;
                        true
                    }
                    _ => false,
                }
            }
            x if x == input::EV_REL => {
                self.event_count += 1;
                let dv = value as i32;
                match code {
                    c if c == input::REL_X => {
                        self.cursor_x = (self.cursor_x + dv).clamp(0, 799);
                        true
                    }
                    c if c == input::REL_Y => {
                        self.cursor_y = (self.cursor_y + dv).clamp(0, 599);
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn draw(&self, fb: &mut Framebuffer) {
        let w = fb.width();
        let h = fb.height();

        // Foni — chuqur ko'k.
        fb.clear(Color::ZAMIN_BG);

        // ---- Top bar ----
        const TOP_H: u32 = 36;
        fb.fill_rect(0, 0, w, TOP_H, Color::rgb(0x14, 0x22, 0x3a));
        fb.hline(0, TOP_H, w, Color::ZAMIN_FG);

        // Logo
        fb.draw_text(12, 8, "ZaminOS", Color::ZAMIN_FG, 2);

        // Joriy app nomi
        let app_text = format!("// {}", self.current_app.label());
        fb.draw_text(180, 12, &app_text, Color::ACCENT, 1);

        // Soat (uptime)
        let clock = format!("uptime  {:02}:{:02}", self.uptime_ticks / 60, self.uptime_ticks % 60);
        let cx = w - (clock.len() as u32) * 8 - 12;
        fb.draw_text(cx, 12, &clock, Color::WHITE, 1);

        // ---- Side panel (app launcher) ----
        const PANEL_W: u32 = 100;
        const PANEL_TOP: u32 = TOP_H + 8;
        let panel_bottom = h - 36;

        fb.fill_rect(0, TOP_H + 1, PANEL_W, h - TOP_H - 1, Color::rgb(0x0c, 0x18, 0x2c));
        fb.vline(PANEL_W, TOP_H + 1, h - TOP_H - 1, Color::ZAMIN_FG);

        let apps = [App::Welcome, App::SysMon, App::Keyboard];
        let mut iy = PANEL_TOP;
        for app in apps.iter() {
            let active = *app == self.current_app;
            let (bg, fg) = if active {
                (Color::ZAMIN_FG, Color::ZAMIN_BG)
            } else {
                (Color::rgb(0x14, 0x22, 0x3a), Color::WHITE)
            };
            fb.fill_rect(10, iy, PANEL_W - 20, 60, bg);
            fb.draw_text(34, iy + 8, app.icon(), fg, 4);
            fb.draw_text(14, iy + 44, app.label(), fg, 1);
            iy += 72;
        }

        // ---- Main content area ----
        let main_x = PANEL_W + 12;
        let main_y = TOP_H + 8;
        let main_w = w - PANEL_W - 24;
        let main_h = panel_bottom - main_y;

        fb.fill_rect(main_x, main_y, main_w, main_h, Color::rgb(0x0a, 0x14, 0x26));
        fb.hline(main_x, main_y, main_w, Color::ACCENT);
        fb.hline(main_x, main_y + main_h - 1, main_w, Color::ACCENT);
        fb.vline(main_x, main_y, main_h, Color::ACCENT);
        fb.vline(main_x + main_w - 1, main_y, main_h, Color::ACCENT);

        match self.current_app {
            App::Welcome => self.draw_welcome(fb, main_x, main_y, main_w, main_h),
            App::SysMon => self.draw_sysmon(fb, main_x, main_y, main_w, main_h),
            App::Keyboard => self.draw_keyboard(fb, main_x, main_y, main_w, main_h),
        }

        // ---- Bottom bar ----
        let bot_y = h - 36;
        fb.fill_rect(0, bot_y, w, 36, Color::rgb(0x14, 0x22, 0x3a));
        fb.hline(0, bot_y, w, Color::ZAMIN_FG);
        fb.draw_text(
            12,
            bot_y + 12,
            "TAB: keyingi app    Yozing: matn      M: mobile/desktop",
            Color::MUTED,
            1,
        );

        // ---- Sichqoncha kursori ----
        if self.cursor_x >= 0 && self.cursor_y >= 0 {
            let cx = self.cursor_x as u32;
            let cy = self.cursor_y as u32;
            fb.fill_rect(cx.saturating_sub(5), cy, 11, 1, Color::ZAMIN_FG);
            fb.fill_rect(cx, cy.saturating_sub(5), 1, 11, Color::ZAMIN_FG);
            fb.put(cx, cy, Color::WHITE);
        }
    }

    fn draw_welcome(&self, fb: &mut Framebuffer, x: u32, y: u32, w: u32, _h: u32) {
        let cx = x + w / 2;
        fb.draw_text(cx - 7 * 8 * 3, y + 30, "ZaminOS", Color::ZAMIN_FG, 6);
        fb.draw_text(cx - 18 * 8, y + 110, "Konvergent OS for aarch64", Color::WHITE, 2);
        fb.draw_text(cx - 21 * 8 / 2, y + 145, "Mobile-first, desktop-ready", Color::ACCENT, 1);

        let lines = [
            "[ OK ]  EL2 -> EL1 transition",
            "[ OK ]  MMU + 4 MiB heap",
            "[ OK ]  GICv2 + generic timer",
            "[ OK ]  Kooperativ scheduler",
            "[ OK ]  ramfb 800x600 framebuffer",
            "[ OK ]  virtio-input (keyboard + tablet)",
            "[ OK ]  Multi-app shell",
        ];
        let mut ly = y + 180;
        for line in lines.iter() {
            fb.draw_text(x + 30, ly, line, Color::SUCCESS, 2);
            ly += 22;
        }
    }

    fn draw_sysmon(&self, fb: &mut Framebuffer, x: u32, y: u32, _w: u32, _h: u32) {
        fb.draw_text(x + 20, y + 20, "System Monitor", Color::ZAMIN_FG, 3);
        fb.hline(x + 20, y + 56, 200, Color::ACCENT);

        let mut row_y = y + 80;
        let stats = [
            ("Architecture", "aarch64".to_string()),
            ("CPU", "cortex-a72 (QEMU virt)".to_string()),
            ("Boot path", "EL2 -> EL1".to_string()),
            ("MMU", "39-bit VA, 1 GiB blocks, identity".to_string()),
            (
                "Heap",
                format!(
                    "{} / {} KiB used  ({} KiB free)",
                    self.heap_used / 1024,
                    self.heap_size / 1024,
                    (self.heap_size - self.heap_used) / 1024
                ),
            ),
            ("Timer", "1 Hz generic timer (PPI 30)".to_string()),
            ("Uptime", format!("{} s", self.uptime_ticks)),
            ("Input events", format!("{}", self.event_count)),
            (
                "Cursor",
                format!("({}, {})", self.cursor_x, self.cursor_y),
            ),
        ];

        for (k, v) in stats.iter() {
            fb.draw_text(x + 24, row_y, k, Color::ACCENT, 1);
            fb.draw_text(x + 200, row_y, v, Color::WHITE, 1);
            row_y += 18;
        }

        // Heap usage bar.
        let bar_x = x + 24;
        let bar_y = row_y + 16;
        let bar_w = 400u32;
        let bar_h = 16u32;
        fb.fill_rect(bar_x, bar_y, bar_w, bar_h, Color::rgb(0x1a, 0x2a, 0x44));
        let frac = if self.heap_size > 0 {
            (self.heap_used as u64 * bar_w as u64 / self.heap_size as u64) as u32
        } else {
            0
        };
        if frac > 0 {
            fb.fill_rect(bar_x, bar_y, frac, bar_h, Color::SUCCESS);
        }
        fb.draw_text(bar_x, bar_y + bar_h + 6, "Heap usage", Color::MUTED, 1);
    }

    fn draw_keyboard(&self, fb: &mut Framebuffer, x: u32, y: u32, w: u32, _h: u32) {
        fb.draw_text(x + 20, y + 20, "Keyboard / Pointer test", Color::ZAMIN_FG, 3);
        fb.hline(x + 20, y + 56, 280, Color::ACCENT);

        fb.draw_text(x + 20, y + 90, "Yozilgan matn:", Color::WHITE, 2);

        // Input box.
        let box_x = x + 20;
        let box_y = y + 120;
        let box_w = w - 40;
        fb.fill_rect(box_x, box_y, box_w, 36, Color::rgb(0x06, 0x0e, 0x1a));
        fb.hline(box_x, box_y, box_w, Color::ZAMIN_FG);
        fb.hline(box_x, box_y + 35, box_w, Color::ZAMIN_FG);
        fb.vline(box_x, box_y, 36, Color::ZAMIN_FG);
        fb.vline(box_x + box_w - 1, box_y, 36, Color::ZAMIN_FG);

        let display = if self.typed.is_empty() {
            "(klaviaturadan yozing)".to_string()
        } else {
            format!("> {}", self.typed)
        };
        let color = if self.typed.is_empty() {
            Color::MUTED
        } else {
            Color::WHITE
        };
        fb.draw_text(box_x + 8, box_y + 9, &display, color, 2);

        // Stats below.
        fb.draw_text(
            x + 20,
            y + 180,
            &format!("Tugmalar / pointer event'lari: {}", self.event_count),
            Color::ACCENT,
            1,
        );
        fb.draw_text(
            x + 20,
            y + 200,
            &format!("Pointer: ({}, {})", self.cursor_x, self.cursor_y),
            Color::ACCENT,
            1,
        );

        // Visual key feedback area
        fb.draw_text(x + 20, y + 240, "Eslatma:", Color::ZAMIN_FG, 2);
        fb.draw_text(x + 20, y + 270, "TAB - keyingi app", Color::MUTED, 1);
        fb.draw_text(x + 20, y + 290, "Backspace - tozalash", Color::MUTED, 1);
        fb.draw_text(x + 20, y + 310, "Tablet harakat - kursor", Color::MUTED, 1);
    }
}

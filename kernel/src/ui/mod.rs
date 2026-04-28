//! ZaminOS shell UI — konvergent layout.
//!
//! Ikkita rejim:
//!   * `Layout::Desktop` — top bar + chap panel + main content (800x600)
//!   * `Layout::Mobile`  — markazda telefon shakli, top status + bottom dock
//!
//! Esc bilan almashtiriladi. Bu Faza 7 ning konvergent va'dasi:
//!   bitta OS, ekran/qurilma o'lchamiga moslashadigan UI.

pub mod apps;

use alloc::format;
use alloc::string::{String, ToString};

use crate::drivers::input;
use crate::graphics::framebuffer::{Color, Framebuffer};

use apps::App;

#[derive(Clone, Copy, PartialEq)]
pub enum Layout {
    Desktop,
    Mobile,
}

impl Layout {
    fn toggle(self) -> Self {
        match self {
            Layout::Desktop => Layout::Mobile,
            Layout::Mobile => Layout::Desktop,
        }
    }
}

pub struct Shell {
    pub layout: Layout,
    pub current_app: App,
    pub uptime_ticks: u32,
    pub event_count: u32,
    pub typed: String,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub heap_used: usize,
    pub heap_size: usize,
    pub terminal: apps::terminal::TerminalState,
    pub paint: apps::paint::PaintState,
}

impl Shell {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            layout: Layout::Desktop,
            current_app: App::Welcome,
            uptime_ticks: 0,
            event_count: 0,
            typed: String::new(),
            cursor_x: (width / 2) as i32,
            cursor_y: (height / 2) as i32,
            heap_used: 0,
            heap_size: 0,
            terminal: apps::terminal::TerminalState::new(),
            paint: apps::paint::PaintState::new(),
        }
    }

    pub fn handle_input(&mut self, ev_type: u16, code: u16, value: u32) -> bool {
        match ev_type {
            x if x == input::EV_KEY => {
                if value != 1 {
                    return false;
                }
                self.event_count += 1;

                // Esc — layout almashtirish
                if code == 1 {
                    self.layout = self.layout.toggle();
                    return true;
                }
                // TAB — keyingi app
                if code == 15 {
                    self.current_app = self.current_app.next();
                    return true;
                }

                // Joriy app keyni yutib qolishi mumkin (masalan terminal Enter).
                if self.current_app == App::Terminal {
                    if self.terminal.handle_key(code) {
                        return true;
                    }
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
                let prev = (self.cursor_x, self.cursor_y);
                match code {
                    c if c == input::ABS_X => {
                        self.cursor_x = ((value as u64 * 800) / 32768) as i32;
                    }
                    c if c == input::ABS_Y => {
                        self.cursor_y = ((value as u64 * 600) / 32768) as i32;
                    }
                    _ => return false,
                }
                if self.current_app == App::Paint {
                    self.paint.on_pointer(prev.0, prev.1, self.cursor_x, self.cursor_y);
                }
                true
            }
            x if x == input::EV_REL => {
                self.event_count += 1;
                let prev = (self.cursor_x, self.cursor_y);
                let dv = value as i32;
                match code {
                    c if c == input::REL_X => {
                        self.cursor_x = (self.cursor_x + dv).clamp(0, 799);
                    }
                    c if c == input::REL_Y => {
                        self.cursor_y = (self.cursor_y + dv).clamp(0, 599);
                    }
                    _ => return false,
                }
                if self.current_app == App::Paint {
                    self.paint.on_pointer(prev.0, prev.1, self.cursor_x, self.cursor_y);
                }
                true
            }
            _ => false,
        }
    }

    pub fn draw(&self, fb: &mut Framebuffer) {
        match self.layout {
            Layout::Desktop => self.draw_desktop(fb),
            Layout::Mobile => self.draw_mobile(fb),
        }

        // Sichqoncha kursori barcha rejimlarda — eng oxirida.
        if self.cursor_x >= 0 && self.cursor_y >= 0 {
            let cx = self.cursor_x as u32;
            let cy = self.cursor_y as u32;
            fb.fill_rect(cx.saturating_sub(5), cy, 11, 1, Color::ZAMIN_FG);
            fb.fill_rect(cx, cy.saturating_sub(5), 1, 11, Color::ZAMIN_FG);
            fb.put(cx, cy, Color::WHITE);
        }
    }

    // -------------------- Desktop layout --------------------

    fn draw_desktop(&self, fb: &mut Framebuffer) {
        let w = fb.width();
        let h = fb.height();

        fb.clear(Color::ZAMIN_BG);

        // Top bar
        const TOP_H: u32 = 36;
        fb.fill_rect(0, 0, w, TOP_H, Color::rgb(0x14, 0x22, 0x3a));
        fb.hline(0, TOP_H, w, Color::ZAMIN_FG);
        fb.draw_text(12, 8, "ZaminOS", Color::ZAMIN_FG, 2);
        let app_text = format!("// {}", self.current_app.label());
        fb.draw_text(180, 12, &app_text, Color::ACCENT, 1);
        let clock = format!("[ DESKTOP ]   uptime  {:02}:{:02}",
            self.uptime_ticks / 60, self.uptime_ticks % 60);
        let cx = w - (clock.len() as u32) * 8 - 12;
        fb.draw_text(cx, 12, &clock, Color::WHITE, 1);

        // Side panel
        const PANEL_W: u32 = 100;
        fb.fill_rect(0, TOP_H + 1, PANEL_W, h - TOP_H - 1, Color::rgb(0x0c, 0x18, 0x2c));
        fb.vline(PANEL_W, TOP_H + 1, h - TOP_H - 1, Color::ZAMIN_FG);

        let apps = App::all();
        let mut iy = TOP_H + 8;
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
            iy += 70;
        }

        // Main area
        let main_x = PANEL_W + 12;
        let main_y = TOP_H + 8;
        let main_w = w - PANEL_W - 24;
        let bot_h = 36;
        let main_h = h - main_y - bot_h - 8;

        fb.fill_rect(main_x, main_y, main_w, main_h, Color::rgb(0x0a, 0x14, 0x26));
        self.frame_box(fb, main_x, main_y, main_w, main_h, Color::ACCENT);
        self.draw_app(fb, main_x, main_y, main_w, main_h);

        // Bottom bar
        let bot_y = h - bot_h;
        fb.fill_rect(0, bot_y, w, bot_h, Color::rgb(0x14, 0x22, 0x3a));
        fb.hline(0, bot_y, w, Color::ZAMIN_FG);
        fb.draw_text(
            12, bot_y + 12,
            "TAB: keyingi app    Esc: mobile/desktop    ',|.': paint clear",
            Color::MUTED, 1,
        );
    }

    // -------------------- Mobile layout --------------------

    fn draw_mobile(&self, fb: &mut Framebuffer) {
        let w = fb.width();
        let h = fb.height();

        // Background — gradient simulation (chuqur ko'k)
        fb.clear(Color::rgb(0x05, 0x0a, 0x16));

        // Side caption (chap)
        fb.draw_text(20, 80, "ZaminOS", Color::ZAMIN_FG, 3);
        fb.draw_text(20, 130, "Mobile mode", Color::WHITE, 2);
        fb.draw_text(20, 160, "(Esc = Desktop)", Color::MUTED, 1);
        fb.draw_text(20, 200, "TAB:  app", Color::ACCENT, 1);
        fb.draw_text(20, 218, "Esc:  mode", Color::ACCENT, 1);

        // Side caption (o'ng)
        let r_x = 600u32;
        fb.draw_text(r_x, 80, "Konvergent", Color::ZAMIN_FG, 2);
        fb.draw_text(r_x, 110, "Bitta OS,", Color::WHITE, 1);
        fb.draw_text(r_x, 128, "Telefon -> PC.", Color::WHITE, 1);
        fb.draw_text(r_x, 150, format!("Uptime: {} s", self.uptime_ticks).as_str(), Color::MUTED, 1);
        fb.draw_text(r_x, 168, format!("Event: {}", self.event_count).as_str(), Color::MUTED, 1);

        // Telefon shakli (bezel)
        let phone_w: u32 = 320;
        let phone_h: u32 = 580;
        let phone_x: u32 = (w - phone_w) / 2;
        let phone_y: u32 = (h - phone_h) / 2;

        // Bezel (qora kontur)
        fb.fill_rect(phone_x - 6, phone_y - 6, phone_w + 12, phone_h + 12, Color::rgb(0x18, 0x18, 0x20));
        // Ekran orqasi
        fb.fill_rect(phone_x, phone_y, phone_w, phone_h, Color::ZAMIN_BG);
        // Ekran chetlari (yorqin chiziq)
        self.frame_box(fb, phone_x, phone_y, phone_w, phone_h, Color::rgb(0x2a, 0x2a, 0x32));

        // Notch (yuqori markazda)
        let notch_w: u32 = 80;
        fb.fill_rect(phone_x + (phone_w - notch_w) / 2, phone_y, notch_w, 14, Color::rgb(0x18, 0x18, 0x20));

        // Status bar (telefon ichida)
        let sb_y = phone_y + 8;
        fb.draw_text(phone_x + 14, sb_y, "9:41", Color::WHITE, 1);
        fb.draw_text(phone_x + phone_w - 60, sb_y, "5G ===", Color::WHITE, 1);

        // App title bar
        let tb_y = phone_y + 28;
        fb.fill_rect(phone_x + 8, tb_y, phone_w - 16, 28, Color::rgb(0x14, 0x22, 0x3a));
        let title = format!("{}  {}", self.current_app.icon(), self.current_app.label());
        fb.draw_text(phone_x + 16, tb_y + 7, &title, Color::ZAMIN_FG, 2);

        // Content area (telefon ichida)
        let content_x = phone_x + 8;
        let content_y = tb_y + 32;
        let content_w = phone_w - 16;
        let dock_h = 80u32;
        let content_h = phone_h - (content_y - phone_y) - dock_h - 8;

        fb.fill_rect(content_x, content_y, content_w, content_h, Color::rgb(0x0a, 0x14, 0x26));
        self.frame_box(fb, content_x, content_y, content_w, content_h, Color::rgb(0x1a, 0x2a, 0x44));
        self.draw_app(fb, content_x, content_y, content_w, content_h);

        // Bottom dock
        let dock_y = phone_y + phone_h - dock_h;
        fb.fill_rect(phone_x + 8, dock_y, phone_w - 16, dock_h - 8, Color::rgb(0x14, 0x22, 0x3a));

        let apps = App::all();
        let n = apps.len() as u32;
        let gap: u32 = 6;
        let avail = phone_w - 16;
        let icon_w = ((avail - (n - 1) * gap) / n).min(48);
        let total_w = n * icon_w + (n - 1) * gap;
        let mut ix = phone_x + phone_w.saturating_sub(total_w) / 2;
        let iy = dock_y + (dock_h - 8 - icon_w) / 2;
        for app in apps.iter() {
            let active = *app == self.current_app;
            let (bg, fg) = if active {
                (Color::ZAMIN_FG, Color::ZAMIN_BG)
            } else {
                (Color::rgb(0x0a, 0x14, 0x26), Color::WHITE)
            };
            fb.fill_rect(ix, iy, icon_w, icon_w, bg);
            // Markazga centered icon harfi
            let icon_text = app.icon();
            let icon_scale = if icon_w >= 36 { 4 } else { 3 };
            let glyph_w = 8 * icon_scale;
            fb.draw_text(
                ix + icon_w.saturating_sub(glyph_w) / 2,
                iy + icon_w.saturating_sub(glyph_w) / 2,
                icon_text,
                fg,
                icon_scale,
            );
            ix += icon_w + gap;
        }

        // Home indicator (pastki polosa)
        let hi_w = 100u32;
        fb.fill_rect(phone_x + (phone_w - hi_w) / 2, phone_y + phone_h - 8, hi_w, 4, Color::WHITE);
    }

    // -------------------- App dispatcher --------------------

    fn draw_app(&self, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
        match self.current_app {
            App::Welcome => apps::welcome::draw(self, fb, x, y, w, h),
            App::SysMon => apps::sysmon::draw(self, fb, x, y, w, h),
            App::Keyboard => apps::keyboard::draw(self, fb, x, y, w, h),
            App::Terminal => apps::terminal::draw(self, fb, x, y, w, h),
            App::Paint => apps::paint::draw(self, fb, x, y, w, h),
            App::Clock => apps::clock::draw(self, fb, x, y, w, h),
        }
    }

    pub fn frame_box(&self, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32, c: Color) {
        fb.hline(x, y, w, c);
        fb.hline(x, y + h - 1, w, c);
        fb.vline(x, y, h, c);
        fb.vline(x + w - 1, y, h, c);
    }
}

#[allow(dead_code)]
fn unused_workaround() -> String {
    String::from("noop")
}

#[allow(dead_code)]
fn _to_string_use() -> String {
    "x".to_string()
}

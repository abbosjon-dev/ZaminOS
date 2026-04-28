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

        // Vertikal gradiyent fon (chuqur ko'k -> yanada chuqurroq)
        fb.vgradient(0, 0, w, h, Color::rgb(0x10, 0x1d, 0x36), Color::rgb(0x06, 0x0c, 0x1a));

        // Top bar — gradiyent bilan
        const TOP_H: u32 = 36;
        fb.vgradient(0, 0, w, TOP_H, Color::rgb(0x1c, 0x2c, 0x4a), Color::rgb(0x12, 0x1e, 0x36));
        fb.hline(0, TOP_H, w, Color::ZAMIN_FG);

        // Logo qutisi
        fb.fill_rect(8, 6, 24, 24, Color::ZAMIN_FG);
        fb.draw_text(12, 10, "Z", Color::ZAMIN_BG, 2);
        fb.draw_text(40, 10, "ZaminOS", Color::WHITE, 2);

        let app_text = format!("//  {}", self.current_app.label());
        fb.draw_text(190, 14, &app_text, Color::ACCENT, 1);

        // Status icons (o'ng tomon): wifi, batareya, soat
        let mut sx = w - 12;
        // Soat
        let clock = format!("{:02}:{:02}", self.uptime_ticks / 60, self.uptime_ticks % 60);
        sx = sx.saturating_sub((clock.len() as u32) * 8);
        fb.draw_text(sx, 14, &clock, Color::WHITE, 1);
        // Batareya
        sx = sx.saturating_sub(34);
        draw_battery_icon(fb, sx, 13);
        // Wi-Fi
        sx = sx.saturating_sub(28);
        draw_wifi_icon(fb, sx, 13);
        // Layout indicator
        sx = sx.saturating_sub(80);
        fb.draw_text(sx, 14, "[ DESKTOP ]", Color::ZAMIN_FG, 1);

        // Side panel — gradiyent
        const PANEL_W: u32 = 100;
        fb.vgradient(0, TOP_H + 1, PANEL_W, h - TOP_H - 1,
            Color::rgb(0x10, 0x1c, 0x32), Color::rgb(0x06, 0x0e, 0x1c));
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
            // Soya
            fb.shadow_rect(10, iy, PANEL_W - 20, 60, Color::rgb(0x02, 0x06, 0x10));
            // Tugma
            fb.round_rect(10, iy, PANEL_W - 20, 60, 8, bg);
            // Active uchun yorqinroq chiziq tepasida
            if active {
                fb.fill_rect(10, iy, 4, 60, Color::rgb(0xff, 0xe0, 0x55));
            }
            draw_app_icon(fb, *app, 34, iy + 6, fg, 4);
            fb.draw_text(14, iy + 44, app.label(), fg, 1);
            iy += 70;
        }

        // Main area — soya + yumaloq panel
        let main_x = PANEL_W + 12;
        let main_y = TOP_H + 8;
        let main_w = w - PANEL_W - 24;
        let bot_h = 30;
        let main_h = h - main_y - bot_h - 8;

        fb.shadow_rect(main_x, main_y, main_w, main_h, Color::rgb(0x02, 0x06, 0x10));
        fb.round_rect(main_x, main_y, main_w, main_h, 10, Color::rgb(0x0a, 0x14, 0x26));
        self.draw_app(fb, main_x, main_y, main_w, main_h);

        // Bottom bar
        let bot_y = h - bot_h;
        fb.vgradient(0, bot_y, w, bot_h, Color::rgb(0x12, 0x1e, 0x36), Color::rgb(0x08, 0x10, 0x20));
        fb.hline(0, bot_y, w, Color::ZAMIN_FG);
        fb.draw_text(12, bot_y + 9,
            "TAB: keyingi app    Esc: mobile/desktop    Tugmalarni bosib sinang",
            Color::MUTED, 1,
        );
    }

    // -------------------- Mobile layout --------------------

    fn draw_mobile(&self, fb: &mut Framebuffer) {
        let w = fb.width();
        let h = fb.height();

        // Workspace fon — gradiyent (purple -> blue)
        fb.vgradient(0, 0, w, h, Color::rgb(0x1a, 0x14, 0x36), Color::rgb(0x06, 0x0c, 0x18));

        // Side caption (chap)
        fb.draw_text(20, 80, "ZaminOS", Color::ZAMIN_FG, 3);
        fb.draw_text(20, 130, "Mobile", Color::WHITE, 3);
        fb.fill_rect(20, 178, 80, 2, Color::ACCENT);
        fb.draw_text(20, 190, "Esc - Desktop", Color::MUTED, 1);
        fb.draw_text(20, 208, "TAB - app", Color::MUTED, 1);

        // Side caption (o'ng)
        let r_x = 610u32;
        fb.draw_text(r_x, 80, "Konvergent", Color::ZAMIN_FG, 2);
        fb.fill_rect(r_x, 102, 110, 2, Color::ACCENT);
        fb.draw_text(r_x, 110, "Bitta OS,", Color::WHITE, 1);
        fb.draw_text(r_x, 126, "Telefon ->", Color::WHITE, 1);
        fb.draw_text(r_x, 142, "Desktop.", Color::WHITE, 1);
        fb.draw_text(r_x, 170, format!("Uptime {}s", self.uptime_ticks).as_str(), Color::MUTED, 1);
        fb.draw_text(r_x, 186, format!("Events {}", self.event_count).as_str(), Color::MUTED, 1);

        // Telefon shakli (bezel)
        let phone_w: u32 = 320;
        let phone_h: u32 = 580;
        let phone_x: u32 = (w - phone_w) / 2;
        let phone_y: u32 = (h - phone_h) / 2;

        // Bezel — soya
        fb.shadow_rect(phone_x - 8, phone_y - 8, phone_w + 16, phone_h + 16, Color::rgb(0x00, 0x00, 0x06));
        fb.round_rect(phone_x - 8, phone_y - 8, phone_w + 16, phone_h + 16, 22, Color::rgb(0x1c, 0x1c, 0x24));
        // Ekran ichi
        fb.round_rect(phone_x, phone_y, phone_w, phone_h, 18, Color::rgb(0x05, 0x0a, 0x18));

        // Notch
        let notch_w: u32 = 90;
        fb.round_rect(phone_x + (phone_w - notch_w) / 2, phone_y - 2, notch_w, 18, 6, Color::rgb(0x10, 0x10, 0x16));

        // Status bar (telefon ichida)
        let sb_y = phone_y + 6;
        fb.draw_text(phone_x + 16, sb_y, "9:41", Color::WHITE, 1);
        // Wi-Fi va batareya o'ngda
        let mut sx = phone_x + phone_w - 8;
        sx = sx.saturating_sub(30);
        draw_battery_icon(fb, sx, sb_y);
        sx = sx.saturating_sub(20);
        draw_wifi_icon(fb, sx, sb_y);
        sx = sx.saturating_sub(16);
        fb.draw_text(sx, sb_y, "5G", Color::WHITE, 1);

        // App title bar
        let tb_y = phone_y + 28;
        fb.round_rect(phone_x + 8, tb_y, phone_w - 16, 28, 6, Color::rgb(0x14, 0x22, 0x3a));
        // Icon
        draw_app_icon(fb, self.current_app, phone_x + 12, tb_y + 4, Color::ZAMIN_FG, 2);
        fb.draw_text(phone_x + 36, tb_y + 7, self.current_app.label(), Color::ZAMIN_FG, 2);

        // Content area (telefon ichida)
        let content_x = phone_x + 8;
        let content_y = tb_y + 32;
        let content_w = phone_w - 16;
        let dock_h = 80u32;
        let content_h = phone_h - (content_y - phone_y) - dock_h - 8;

        fb.round_rect(content_x, content_y, content_w, content_h, 8, Color::rgb(0x08, 0x12, 0x22));
        self.draw_app(fb, content_x, content_y, content_w, content_h);

        // Bottom dock — yumaloq
        let dock_y = phone_y + phone_h - dock_h;
        fb.round_rect(phone_x + 8, dock_y, phone_w - 16, dock_h - 8, 12, Color::rgb(0x14, 0x22, 0x3a));

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
                (Color::rgb(0x0c, 0x18, 0x2c), Color::WHITE)
            };
            fb.round_rect(ix, iy, icon_w, icon_w, 6, bg);
            // Vizual ikona — markazlashtirilgan
            let icon_scale = if icon_w >= 40 { 4 } else { 3 };
            let glyph_w = 8 * icon_scale;
            draw_app_icon(
                fb,
                *app,
                ix + icon_w.saturating_sub(glyph_w) / 2,
                iy + icon_w.saturating_sub(glyph_w) / 2,
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

// --- Vizual ikonalar ---

/// App ikonasi: oddiy harf o'rniga app turiga mos kichik grafik.
pub fn draw_app_icon(fb: &mut Framebuffer, app: App, x: u32, y: u32, fg: Color, scale: u32) {
    let s = scale; // har piksel s × s
    let g = |i: u32, j: u32| (x + i * s, y + j * s);

    // Glyph 8x8 grid — sodda piktogrammalar.
    let pattern: &[u8] = match app {
        App::Welcome => &[
            0b00011000,
            0b00111100,
            0b01100110,
            0b11000011,
            0b11000011,
            0b01111110,
            0b01100110,
            0b01100110,
        ], // home/house
        App::SysMon => &[
            0b00000000,
            0b00010000,
            0b00010100,
            0b00111100,
            0b01010110,
            0b01111111,
            0b00000000,
            0b11111111,
        ], // bar chart
        App::Keyboard => &[
            0b00000000,
            0b11111111,
            0b10101011,
            0b10000001,
            0b10101011,
            0b10000001,
            0b11111111,
            0b00000000,
        ], // keys
        App::Terminal => &[
            0b11111111,
            0b10000001,
            0b10110001,
            0b10011001,
            0b10001101,
            0b10000001,
            0b10111101,
            0b11111111,
        ], // terminal box
        App::Paint => &[
            0b00111100,
            0b01000010,
            0b10100101,
            0b10100101,
            0b10000001,
            0b01000010,
            0b00011100,
            0b00001000,
        ], // palette
        App::Clock => &[
            0b00111100,
            0b01000010,
            0b10001001,
            0b10001001,
            0b10001111,
            0b10000001,
            0b01000010,
            0b00111100,
        ], // clock
    };

    for (row, byte) in pattern.iter().enumerate() {
        for col in 0..8u32 {
            // High bit = leftmost (col 0)
            if byte & (1 << (7 - col)) != 0 {
                let (px, py) = g(col, row as u32);
                fb.fill_rect(px, py, s, s, fg);
            }
        }
    }
}

/// Wi-Fi ikona — uchburchak signal.
pub fn draw_wifi_icon(fb: &mut Framebuffer, x: u32, y: u32) {
    let c = Color::WHITE;
    // 14x10 area
    // Pastki nuqta
    fb.fill_rect(x + 6, y + 8, 2, 2, c);
    // Birinchi yoy
    fb.fill_rect(x + 4, y + 5, 6, 1, c);
    fb.put(x + 3, y + 6, c);
    fb.put(x + 10, y + 6, c);
    // Ikkinchi yoy
    fb.fill_rect(x + 2, y + 2, 10, 1, c);
    fb.put(x + 1, y + 3, c);
    fb.put(x + 12, y + 3, c);
    // Uchinchi yoy (eng katta)
    fb.fill_rect(x, y, 14, 1, c);
}

/// Batareya ikona — o'ng tomonda kichik uch va asosiy quti.
pub fn draw_battery_icon(fb: &mut Framebuffer, x: u32, y: u32) {
    let c = Color::WHITE;
    let g = Color::SUCCESS;
    // Tashqi quti 22x10
    fb.hline(x, y + 1, 22, c);
    fb.hline(x, y + 8, 22, c);
    fb.vline(x, y + 1, 8, c);
    fb.vline(x + 21, y + 1, 8, c);
    // Uchi (right)
    fb.fill_rect(x + 22, y + 3, 2, 4, c);
    // Quvvat (~80%)
    fb.fill_rect(x + 2, y + 3, 16, 4, g);
}

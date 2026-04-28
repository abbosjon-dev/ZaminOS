//! ZaminOS shell UI — konvergent layout.
//!
//! Ikkita rejim:
//!   * `Layout::Desktop` — top bar + chap panel + main content (800x600)
//!   * `Layout::Mobile`  — markazda telefon shakli, top status + bottom dock
//!
//! Esc bilan almashtiriladi. Bu Faza 7 ning konvergent va'dasi:
//!   bitta OS, ekran/qurilma o'lchamiga moslashadigan UI.

pub mod apps;
pub mod desktop;
pub mod icons;
pub mod mobile;

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

#[derive(Clone, Copy, PartialEq)]
pub enum MobileView {
    /// Lock screen — boot oxirida ko'rinadi.
    Lock,
    /// Home screen — widgetlar va dock.
    Home,
    /// App drawer — barcha apps grid.
    AppDrawer,
    /// App ichida.
    InApp,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DesktopView {
    /// Asosiy ko'rinish — desktop wallpaper + widgetlar + open window.
    Desktop,
    /// Start menu ochiq.
    StartMenu,
}

pub struct Shell {
    pub layout: Layout,
    pub mobile_view: MobileView,
    pub desktop_view: DesktopView,
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
    pub calculator: apps::calculator::CalcState,
    pub files: apps::files::FilesState,
    pub music: apps::music::MusicState,
    pub settings: apps::settings::SettingsState,
}

impl Shell {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            layout: Layout::Desktop,
            mobile_view: MobileView::Lock,
            desktop_view: DesktopView::Desktop,
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
            calculator: apps::calculator::CalcState::new(),
            files: apps::files::FilesState::new(),
            music: apps::music::MusicState::new(),
            settings: apps::settings::SettingsState::new(),
        }
    }

    pub fn handle_input(&mut self, ev_type: u16, code: u16, value: u32) -> bool {
        match ev_type {
            x if x == input::EV_KEY => {
                if value != 1 {
                    return false;
                }
                self.event_count += 1;

                // Esc — layout almashtirish (mobile -> lock screen)
                if code == 1 {
                    self.layout = self.layout.toggle();
                    if self.layout == Layout::Mobile {
                        self.mobile_view = MobileView::Lock;
                    }
                    return true;
                }

                // Mobile lock screen: Enter (yoki har qanday tugma) — Home'ga
                if self.layout == Layout::Mobile && self.mobile_view == MobileView::Lock {
                    self.mobile_view = MobileView::Home;
                    return true;
                }

                // F1 — mobile rejimda "Home" tugmasi
                if code == 59 && self.layout == Layout::Mobile {
                    self.mobile_view = MobileView::Home;
                    return true;
                }

                // F2 — mobile App drawer (Android menu)
                if code == 60 && self.layout == Layout::Mobile {
                    self.mobile_view = MobileView::AppDrawer;
                    return true;
                }

                // F2 — desktop Start menu toggle (Windows)
                if code == 60 && self.layout == Layout::Desktop {
                    self.desktop_view = match self.desktop_view {
                        DesktopView::Desktop => DesktopView::StartMenu,
                        DesktopView::StartMenu => DesktopView::Desktop,
                    };
                    return true;
                }

                // Mobile Home/AppDrawer: Tab cikl, Enter ishga tushirish
                if self.layout == Layout::Mobile
                    && (self.mobile_view == MobileView::Home || self.mobile_view == MobileView::AppDrawer)
                {
                    if code == 15 {
                        self.current_app = self.current_app.next();
                        return true;
                    }
                    if code == 28 {
                        self.mobile_view = MobileView::InApp;
                        return true;
                    }
                    return false;
                }

                // Desktop StartMenu: Tab cikl, Enter ochish
                if self.layout == Layout::Desktop && self.desktop_view == DesktopView::StartMenu {
                    if code == 15 {
                        self.current_app = self.current_app.next();
                        return true;
                    }
                    if code == 28 {
                        self.desktop_view = DesktopView::Desktop;
                        return true;
                    }
                    return false;
                }

                // TAB — keyingi app (desktop yoki mobile InApp)
                if code == 15 {
                    self.current_app = self.current_app.next();
                    return true;
                }

                // Joriy app keyni yutib qolishi mumkin.
                match self.current_app {
                    App::Terminal => if self.terminal.handle_key(code) { return true; },
                    App::Calculator => if self.calculator.handle_key(code) { return true; },
                    App::Files => if self.files.handle_key(code) { return true; },
                    App::Music => if self.music.handle_key(code) { return true; },
                    App::Settings => if self.settings.handle_key(code) { return true; },
                    _ => {}
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
        // Windows 11 uslubli desktop
        crate::ui::desktop::draw(self, fb);
    }

    fn draw_desktop_OLD(&self, fb: &mut Framebuffer) {
        let w = fb.width();
        let h = fb.height();

        // ---- Wallpaper: ko'k -> binafsha gradient ----
        fb.vgradient(0, 0, w, h, Color::rgb(0x1e, 0x1a, 0x4a), Color::rgb(0x05, 0x0a, 0x1e));

        // ---- Menu bar (macOS-uslubli) ----
        const MENU_H: u32 = 28;
        fb.fill_rect(0, 0, w, MENU_H, Color::rgb(0x14, 0x18, 0x28));
        fb.hline(0, MENU_H, w, Color::rgb(0x30, 0x36, 0x4a));

        // Apple-style logo qutisi (sariq Z)
        fb.fill_rect(10, 5, 18, 18, Color::ZAMIN_FG);
        fb.draw_text(13, 9, "Z", Color::ZAMIN_BG, 2);

        // App name (bold)
        fb.draw_text(38, 8, "ZaminOS", Color::WHITE, 2);

        // App menus
        let menu_items = ["File", "Edit", "View", "Window", "Help"];
        let mut mx = 168u32;
        for item in menu_items.iter() {
            fb.draw_text(mx, 10, item, Color::rgb(0xc0, 0xc4, 0xd0), 1);
            mx += (item.len() as u32) * 8 + 16;
        }

        // Status icons (o'ng)
        let mut sx = w - 12;
        let clock = format!("{:02}:{:02}", self.uptime_ticks / 60, self.uptime_ticks % 60);
        sx = sx.saturating_sub((clock.len() as u32) * 8);
        fb.draw_text(sx, 10, &clock, Color::WHITE, 1);
        sx = sx.saturating_sub(34);
        draw_battery_icon(fb, sx, 9);
        sx = sx.saturating_sub(24);
        draw_wifi_icon(fb, sx, 9);
        sx = sx.saturating_sub(40);
        fb.draw_text(sx, 10, "DESKTOP", Color::ZAMIN_FG, 1);

        // ---- Window (joriy app) ----
        const DOCK_H: u32 = 76;
        let win_x = 24u32;
        let win_y = MENU_H + 18;
        let win_w = w - 48;
        let win_h = h - MENU_H - 18 - DOCK_H - 10;

        // Soya
        fb.shadow_rect(win_x, win_y + 4, win_w, win_h, Color::rgb(0x00, 0x00, 0x06));

        // Window background (dark navy with rounded corners)
        fb.round_rect(win_x, win_y, win_w, win_h, 10, Color::rgb(0x0a, 0x14, 0x26));

        // Window title bar
        const TITLE_H: u32 = 28;
        // Top of window has its own slightly lighter band
        // ... but only above content, with rounded top
        for j in 0..TITLE_H {
            let row_clip = if j < 10 {
                let dy = (10 - j) as i32;
                let dx2 = 100i32 - dy * dy;
                10 - (isqrt(dx2.max(0) as u32) as i32).max(0)
            } else { 0 };
            let off = row_clip.max(0) as u32;
            fb.fill_rect(
                win_x + off, win_y + j,
                win_w - 2 * off, 1,
                Color::rgb(0x18, 0x22, 0x3c),
            );
        }
        fb.hline(win_x, win_y + TITLE_H, win_w, Color::rgb(0x30, 0x36, 0x4a));

        // Traffic light buttons (red/yellow/green)
        let tl_y = win_y + 9;
        let tl_r = 6u32;
        let tl_x_red = win_x + 14;
        let tl_x_yel = tl_x_red + 18;
        let tl_x_grn = tl_x_yel + 18;
        fb.fill_circle((tl_x_red + tl_r) as i32, (tl_y + tl_r) as i32, tl_r, Color::rgb(0xff, 0x5f, 0x57));
        fb.fill_circle((tl_x_yel + tl_r) as i32, (tl_y + tl_r) as i32, tl_r, Color::rgb(0xfe, 0xbc, 0x2e));
        fb.fill_circle((tl_x_grn + tl_r) as i32, (tl_y + tl_r) as i32, tl_r, Color::rgb(0x28, 0xc8, 0x40));

        // Window title (centered)
        let title = self.current_app.label();
        let title_w = (title.len() as u32) * 8 * 2;
        fb.draw_text(
            win_x + (win_w.saturating_sub(title_w)) / 2,
            win_y + 6,
            title,
            Color::WHITE,
            2,
        );

        // Window content area
        let content_x = win_x + 1;
        let content_y = win_y + TITLE_H + 1;
        let content_w = win_w - 2;
        let content_h = win_h - TITLE_H - 2;
        self.draw_app(fb, content_x, content_y, content_w, content_h);

        // ---- Dock (macOS-uslubli, pastda markazda) ----
        let apps = App::all();
        let n = apps.len() as u32;
        let dock_icon = 56u32;
        let dock_pad = 12u32;
        let dock_gap = 8u32;
        let dock_w = n * dock_icon + (n - 1) * dock_gap + 2 * dock_pad;
        let dock_h_actual = dock_icon + 2 * dock_pad;
        let dock_x = (w - dock_w) / 2;
        let dock_y = h - dock_h_actual - 8;

        // Dock fon (yarim shaffof dark)
        fb.shadow_rect(dock_x, dock_y + 4, dock_w, dock_h_actual, Color::rgb(0x00, 0x00, 0x06));
        fb.round_rect(dock_x, dock_y, dock_w, dock_h_actual, 16, Color::rgb(0x18, 0x22, 0x3c));

        let mut ix = dock_x + dock_pad;
        let iy = dock_y + dock_pad;
        for app in apps.iter() {
            let active = *app == self.current_app;
            // Active uchun pastida nuqta
            if active {
                fb.fill_circle(
                    (ix + dock_icon / 2) as i32,
                    (iy + dock_icon + 6) as i32,
                    2,
                    Color::WHITE,
                );
            }
            draw_app_icon_modern(fb, *app, ix, iy, dock_icon);
            ix += dock_icon + dock_gap;
        }
    }

    // -------------------- Mobile layout --------------------

    fn draw_mobile(&self, fb: &mut Framebuffer) {
        // Android uslubli mobile
        crate::ui::mobile::draw(self, fb);
    }

    fn draw_mobile_OLD(&self, fb: &mut Framebuffer) {
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

        // Mobile view ga qarab content area:
        //   Home  — app grid
        //   InApp — joriy app + title bar
        let dock_h = 76u32;

        match self.mobile_view {
            MobileView::Lock | MobileView::AppDrawer => {} // new mobile.rs handles these
            MobileView::Home => {
                // Status'tan keyin to'g'ridan-to'g'ri grid
                let grid_y = phone_y + 26;
                let grid_h = phone_h - 26 - dock_h - 8;
                draw_home_grid(fb, phone_x + 8, grid_y, phone_w - 16, grid_h, self.current_app);
            }
            MobileView::InApp => {
                // App title bar
                let tb_y = phone_y + 28;
                fb.round_rect(
                    phone_x + 8, tb_y, phone_w - 16, 28, 6,
                    Color::rgb(0x14, 0x22, 0x3a),
                );
                draw_app_icon_modern(fb, self.current_app, phone_x + 14, tb_y - 4, 36);
                fb.draw_text(phone_x + 56, tb_y + 7, self.current_app.label(), Color::WHITE, 2);
                // Home button (F1) hint
                fb.draw_text(phone_x + phone_w - 50, tb_y + 9, "[F1]", Color::MUTED, 1);

                let content_x = phone_x + 8;
                let content_y = tb_y + 36;
                let content_w = phone_w - 16;
                let content_h = phone_h - (content_y - phone_y) - dock_h - 8;
                fb.round_rect(
                    content_x, content_y, content_w, content_h, 8,
                    Color::rgb(0x08, 0x12, 0x22),
                );
                self.draw_app(fb, content_x, content_y, content_w, content_h);
            }
        }

        // Bottom dock — yumaloq
        let dock_y = phone_y + phone_h - dock_h;
        fb.round_rect(phone_x + 8, dock_y, phone_w - 16, dock_h - 8, 14, Color::rgb(0x14, 0x22, 0x3a));

        let apps = App::all();
        let n = apps.len() as u32;
        let gap: u32 = 8;
        let avail = phone_w - 24;
        let icon_w = ((avail - (n - 1) * gap) / n).min(46);
        let total_w = n * icon_w + (n - 1) * gap;
        let mut ix = phone_x + phone_w.saturating_sub(total_w) / 2;
        let iy = dock_y + (dock_h - 8 - icon_w) / 2;
        for app in apps.iter() {
            let active = *app == self.current_app;
            // Active uchun pastida ko'rinadigan nuqta
            if active {
                fb.fill_circle(
                    (ix + icon_w / 2) as i32,
                    (iy + icon_w + 6) as i32,
                    2,
                    Color::WHITE,
                );
            }
            // Rangli gradient ikon
            draw_app_icon_modern(fb, *app, ix, iy, icon_w);
            ix += icon_w + gap;
        }

        // Home indicator (pastki polosa)
        let hi_w = 100u32;
        fb.fill_rect(phone_x + (phone_w - hi_w) / 2, phone_y + phone_h - 8, hi_w, 4, Color::WHITE);
    }

    // -------------------- App dispatcher --------------------

    /// Public dispatcher for desktop/mobile modules.
    pub fn draw_app_into(&self, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
        self.draw_app(fb, x, y, w, h);
    }

    fn draw_app(&self, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
        match self.current_app {
            App::Welcome => apps::welcome::draw(self, fb, x, y, w, h),
            App::SysMon => apps::sysmon::draw(self, fb, x, y, w, h),
            App::Calculator => apps::calculator::draw(self, fb, x, y, w, h),
            App::Keyboard => apps::keyboard::draw(self, fb, x, y, w, h),
            App::Terminal => apps::terminal::draw(self, fb, x, y, w, h),
            App::Paint => apps::paint::draw(self, fb, x, y, w, h),
            App::Files => apps::files::draw(self, fb, x, y, w, h),
            App::Music => apps::music::draw(self, fb, x, y, w, h),
            App::Clock => apps::clock::draw(self, fb, x, y, w, h),
            App::Settings => apps::settings::draw(self, fb, x, y, w, h),
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

/// App ikonasi 8x8 piktogramma (oq glyph).
fn icon_pattern(app: App) -> &'static [u8; 8] {
    match app {
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
        ], // terminal
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
        App::Calculator => &[
            0b11111111,
            0b10000001,
            0b10111101,
            0b10000001,
            0b10101010,
            0b10101010,
            0b10101010,
            0b11111111,
        ], // calc grid
        App::Files => &[
            0b00111100,
            0b01000110,
            0b10000010,
            0b10000010,
            0b10000010,
            0b10000010,
            0b10000010,
            0b11111110,
        ], // folder
        App::Music => &[
            0b00011110,
            0b00010010,
            0b00010010,
            0b00010010,
            0b00010010,
            0b01110010,
            0b11110010,
            0b11100000,
        ], // music note
        App::Settings => &[
            0b00011000,
            0b01011010,
            0b00111100,
            0b11100111,
            0b11100111,
            0b00111100,
            0b01011010,
            0b00011000,
        ], // gear-ish
    }
}

/// Zamonaviy rangli ikona: gradient rounded square + oq piktogramma.
/// `size` — ikon yon tomoni piksellarda. Glyph markazlashtiriladi.
pub fn draw_app_icon_modern(fb: &mut Framebuffer, app: App, x: u32, y: u32, size: u32) {
    // Soya
    fb.shadow_rect(x, y + 2, size, size, Color::rgb(0x00, 0x00, 0x06));
    // Gradient fon
    draw_round_gradient(fb, x, y, size, size, size / 5, app.tint_top(), app.tint_bottom());

    // Glyph — markazda
    let glyph_size = (size * 5 / 8).max(8);
    let glyph_scale = (glyph_size / 8).max(1);
    let glyph_w = 8 * glyph_scale;
    let glyph_x = x + (size - glyph_w) / 2;
    let glyph_y = y + (size - glyph_w) / 2;
    let pattern = icon_pattern(app);
    for (row, byte) in pattern.iter().enumerate() {
        for col in 0..8u32 {
            if byte & (1 << (7 - col)) != 0 {
                fb.fill_rect(
                    glyph_x + col * glyph_scale,
                    glyph_y + row as u32 * glyph_scale,
                    glyph_scale,
                    glyph_scale,
                    Color::WHITE,
                );
            }
        }
    }
}

/// Yumaloq burchakli vertikal gradient — radius bilan.
fn draw_round_gradient(
    fb: &mut Framebuffer,
    x: u32, y: u32, w: u32, h: u32, r: u32,
    top: Color, bottom: Color,
) {
    if w <= 2 * r || h <= 2 * r {
        fb.vgradient(x, y, w, h, top, bottom);
        return;
    }
    let (tr, tg, tb) = ((top.0 >> 16) & 0xff, (top.0 >> 8) & 0xff, top.0 & 0xff);
    let (br, bg, bb) = ((bottom.0 >> 16) & 0xff, (bottom.0 >> 8) & 0xff, bottom.0 & 0xff);
    let r_i = r as i32;
    let r2 = r_i * r_i;

    for j in 0..h {
        let t = j as i32;
        let total = (h - 1).max(1) as i32;
        let red = (tr as i32 + (br as i32 - tr as i32) * t / total) as u32;
        let grn = (tg as i32 + (bg as i32 - tg as i32) * t / total) as u32;
        let blu = (tb as i32 + (bb as i32 - tb as i32) * t / total) as u32;
        let c = Color((red << 16) | (grn << 8) | blu);

        // Boshlang'ich/oxirgi piksel x — burchaklarda kichraytirilgan
        let row_offset = if j < r {
            // Yuqori burchak: dy = r - j, dx = sqrt(r^2 - dy^2)
            let dy = (r - j) as i32;
            let dx2 = r2 - dy * dy;
            r_i - (isqrt(dx2 as u32) as i32).max(0)
        } else if j >= h - r {
            let dy = (j - (h - r - 1)) as i32;
            let dx2 = r2 - dy * dy;
            r_i - (isqrt(dx2.max(0) as u32) as i32).max(0)
        } else {
            0
        };
        let row_offset = row_offset.max(0) as u32;
        if row_offset < w / 2 {
            fb.fill_rect(x + row_offset, y + j, w - 2 * row_offset, 1, c);
        }
    }
}

fn isqrt(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Mobile home screen — iOS uslubli app grid.
/// `selected` — TAB bilan tanlangan app (yorqin highlight).
pub fn draw_home_grid(fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32, selected: App) {
    // Sarlavha
    fb.draw_text(x + 12, y + 4, "Home", Color::WHITE, 2);
    fb.draw_text(x + 12, y + 24, "Tap an app", Color::MUTED, 1);

    let grid_top = y + 50;
    let cols: u32 = 3;
    let rows: u32 = 2;
    let icon = 64u32;
    let h_gap = 12u32;
    let v_gap = 24u32;
    let total_w = cols * icon + (cols - 1) * h_gap;
    let start_x = x + (w.saturating_sub(total_w)) / 2;

    let apps = App::all();
    for (idx, app) in apps.iter().enumerate() {
        let r = (idx as u32) / cols;
        let c = (idx as u32) % cols;
        if r >= rows {
            break;
        }
        let ix = start_x + c * (icon + h_gap);
        let iy = grid_top + r * (icon + 22 + v_gap);

        // Selection highlight ring
        if *app == selected {
            fb.round_rect(
                ix.saturating_sub(4),
                iy.saturating_sub(4),
                icon + 8,
                icon + 8,
                14,
                Color::rgb(0xff, 0xe0, 0x55),
            );
        }
        draw_app_icon_modern(fb, *app, ix, iy, icon);
        // Label
        let lbl = app.label();
        let lw = (lbl.len() as u32) * 8;
        fb.draw_text(ix + (icon - lw) / 2, iy + icon + 6, lbl, Color::WHITE, 1);
    }

    // Page dots (bittagina sahifa)
    let dot_y = y + h - 14;
    let dot_x = x + w / 2;
    fb.fill_circle(dot_x as i32, dot_y as i32, 3, Color::WHITE);
    fb.fill_circle(dot_x as i32 + 14, dot_y as i32, 3, Color::rgb(0x55, 0x55, 0x66));
}

/// Eski oddiy ikon (glyph-only, single color) — backward compat uchun.
pub fn draw_app_icon(fb: &mut Framebuffer, app: App, x: u32, y: u32, fg: Color, scale: u32) {
    let pattern = icon_pattern(app);
    for (row, byte) in pattern.iter().enumerate() {
        for col in 0..8u32 {
            if byte & (1 << (7 - col)) != 0 {
                fb.fill_rect(x + col * scale, y + row as u32 * scale, scale, scale, fg);
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

//! Shell applari.

pub mod calculator;
pub mod clock;
pub mod files;
pub mod keyboard;
pub mod music;
pub mod paint;
pub mod settings;
pub mod sysmon;
pub mod terminal;
pub mod welcome;

use crate::graphics::framebuffer::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum App {
    Welcome,
    SysMon,
    Calculator,
    Keyboard,
    Terminal,
    Paint,
    Files,
    Music,
    Clock,
    Settings,
}

impl App {
    pub fn all() -> [App; 10] {
        [
            App::Welcome,
            App::SysMon,
            App::Calculator,
            App::Keyboard,
            App::Terminal,
            App::Paint,
            App::Files,
            App::Music,
            App::Clock,
            App::Settings,
        ]
    }

    pub fn next(self) -> Self {
        let all = Self::all();
        let idx = all.iter().position(|a| *a == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            App::Welcome => "Welcome",
            App::SysMon => "Activity",
            App::Calculator => "Calc",
            App::Keyboard => "Keys",
            App::Terminal => "Terminal",
            App::Paint => "Paint",
            App::Files => "Files",
            App::Music => "Music",
            App::Clock => "Clock",
            App::Settings => "Settings",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            App::Welcome => "Z",
            App::SysMon => "S",
            App::Calculator => "=",
            App::Keyboard => "K",
            App::Terminal => ">",
            App::Paint => "P",
            App::Files => "F",
            App::Music => "M",
            App::Clock => "C",
            App::Settings => "@",
        }
    }

    /// Ikon foni — gradient yuqori qismidagi rang.
    pub fn tint_top(self) -> Color {
        match self {
            App::Welcome => Color::rgb(0xff, 0xc7, 0x4c),
            App::SysMon => Color::rgb(0x4c, 0xaf, 0x50),
            App::Calculator => Color::rgb(0xff, 0x9f, 0x0a),
            App::Keyboard => Color::rgb(0x42, 0xa5, 0xf5),
            App::Terminal => Color::rgb(0x37, 0x47, 0x4f),
            App::Paint => Color::rgb(0xab, 0x47, 0xbc),
            App::Files => Color::rgb(0x29, 0xb6, 0xf6),
            App::Music => Color::rgb(0xec, 0x40, 0x7a),
            App::Clock => Color::rgb(0xef, 0x53, 0x50),
            App::Settings => Color::rgb(0x78, 0x90, 0x9c),
        }
    }

    /// Ikon foni — gradient pastki rangi (chuqurroq).
    pub fn tint_bottom(self) -> Color {
        match self {
            App::Welcome => Color::rgb(0xe6, 0x91, 0x10),
            App::SysMon => Color::rgb(0x2e, 0x7d, 0x32),
            App::Calculator => Color::rgb(0xc6, 0x6f, 0x00),
            App::Keyboard => Color::rgb(0x15, 0x65, 0xc0),
            App::Terminal => Color::rgb(0x1c, 0x25, 0x30),
            App::Paint => Color::rgb(0x6a, 0x1b, 0x9a),
            App::Files => Color::rgb(0x02, 0x77, 0xbd),
            App::Music => Color::rgb(0xad, 0x14, 0x57),
            App::Clock => Color::rgb(0xc6, 0x28, 0x28),
            App::Settings => Color::rgb(0x45, 0x5a, 0x64),
        }
    }
}


//! Shell applari.

pub mod clock;
pub mod keyboard;
pub mod paint;
pub mod sysmon;
pub mod terminal;
pub mod welcome;

#[derive(Clone, Copy, PartialEq)]
pub enum App {
    Welcome,
    SysMon,
    Keyboard,
    Terminal,
    Paint,
    Clock,
}

impl App {
    pub fn all() -> [App; 6] {
        [
            App::Welcome,
            App::SysMon,
            App::Keyboard,
            App::Terminal,
            App::Paint,
            App::Clock,
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
            App::SysMon => "SysMon",
            App::Keyboard => "Keyboard",
            App::Terminal => "Terminal",
            App::Paint => "Paint",
            App::Clock => "Clock",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            App::Welcome => "Z",
            App::SysMon => "S",
            App::Keyboard => "K",
            App::Terminal => ">",
            App::Paint => "P",
            App::Clock => "C",
        }
    }
}

//! Calculator app — to'liq ishlovchi tugmalar gridi bilan.

use alloc::format;
use alloc::string::String;

use crate::drivers::input;
use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => if b == 0.0 { 0.0 } else { a / b },
        }
    }
    fn label(self) -> &'static str {
        match self { Op::Add => "+", Op::Sub => "-", Op::Mul => "*", Op::Div => "/" }
    }
}

pub struct CalcState {
    /// Hozir ekranda ko'ringan satr.
    display: String,
    /// Avvalgi operand (yig'ilgan natija).
    accum: f64,
    /// Hozirgi operatsiya kutilmoqda.
    pending: Option<Op>,
    /// Yangi sonni boshlash uchun bayroq.
    fresh: bool,
}

impl CalcState {
    pub fn new() -> Self {
        Self {
            display: String::from("0"),
            accum: 0.0,
            pending: None,
            fresh: true,
        }
    }

    pub fn handle_key(&mut self, code: u16) -> bool {
        // Raqamlar
        let digit = match code {
            2 => Some('1'), 3 => Some('2'), 4 => Some('3'),
            5 => Some('4'), 6 => Some('5'), 7 => Some('6'),
            8 => Some('7'), 9 => Some('8'), 10 => Some('9'),
            11 => Some('0'),
            _ => None,
        };
        if let Some(d) = digit {
            if self.fresh {
                self.display.clear();
                self.fresh = false;
            }
            if self.display == "0" {
                self.display.clear();
            }
            self.display.push(d);
            return true;
        }
        // 'p' = + ('plus'), 'm' = - ('minus'), 'x' = * ('times'), 'd' = / ('divide')
        let op = match code {
            // Aniq tugmalarga 'a','s','d','f' beramiz (h/j/k/l)
            36 => Some(Op::Add),  // 'j' -> +
            37 => Some(Op::Sub),  // 'k' -> -
            38 => Some(Op::Mul),  // 'l' -> *
            35 => Some(Op::Div),  // 'h' -> /
            _ => None,
        };
        if let Some(o) = op {
            self.compute_pending();
            self.pending = Some(o);
            self.fresh = true;
            return true;
        }
        // Enter = '='
        if code == 28 {
            self.compute_pending();
            self.pending = None;
            self.fresh = true;
            return true;
        }
        // Backspace
        if code == 14 {
            if self.fresh {
                self.display = String::from("0");
            } else {
                self.display.pop();
                if self.display.is_empty() {
                    self.display.push('0');
                    self.fresh = true;
                }
            }
            return true;
        }
        // 'c' = clear
        if code == 46 {
            self.display = String::from("0");
            self.accum = 0.0;
            self.pending = None;
            self.fresh = true;
            return true;
        }
        false
    }

    fn compute_pending(&mut self) {
        let cur: f64 = self.display.parse().unwrap_or(0.0);
        if let Some(op) = self.pending {
            let r = op.apply(self.accum, cur);
            self.accum = r;
            self.display = format_num(r);
        } else {
            self.accum = cur;
        }
    }
}

fn format_num(n: f64) -> String {
    // Sodda — agar butun bo'lsa butun, aks holda 4 raqamgacha.
    if (n - (n as i64 as f64)).abs() < 1e-9 {
        format!("{}", n as i64)
    } else {
        // Manual 4-decimal format (Rust f64::Display talab qiladi a lot)
        let scaled = (n * 10000.0 + if n >= 0.0 { 0.5 } else { -0.5 }) as i64;
        let int = scaled / 10000;
        let frac = (scaled.abs() % 10000) as u32;
        let s = format!("{}.{:04}", int, frac);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

use alloc::string::ToString;

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // Title bar
    fb.fill_rect(x, y, w, 22, Color::rgb(0x14, 0x22, 0x3a));
    fb.draw_text(x + 8, y + 6, "Calculator  -  raqamlar va j/k/l/h, Enter = =", Color::ZAMIN_FG, 1);

    // Display panel
    let disp_y = y + 28;
    let disp_h = 60u32;
    fb.round_rect(x + 12, disp_y, w - 24, disp_h, 8, Color::rgb(0x05, 0x0a, 0x16));
    let display = &shell.calculator.display;
    let scale = if display.len() <= 10 { 4 } else if display.len() <= 16 { 3 } else { 2 };
    let dw = (display.len() as u32) * 8 * scale;
    fb.draw_text(
        x + w - dw - 24,
        disp_y + (disp_h - 8 * scale) / 2,
        display,
        Color::WHITE,
        scale,
    );

    if let Some(op) = shell.calculator.pending {
        fb.draw_text(x + 24, disp_y + 8, op.label(), Color::ACCENT, 3);
    }

    // Tugmalar gridi
    let grid_y = disp_y + disp_h + 16;
    let grid_h = (y + h).saturating_sub(grid_y).saturating_sub(8);
    let cols = 4;
    let rows = 5;
    let gap = 6u32;
    let avail_w = w - 24;
    let avail_h = grid_h.saturating_sub(8);
    let bw = (avail_w - (cols - 1) * gap) / cols;
    let bh = (avail_h - (rows - 1) * gap) / rows;

    // Tugmalar matritsasi: nom + rang turi
    let buttons: [[(&str, ButtonKind); 4]; 5] = [
        [("C", ButtonKind::Action), ("<-", ButtonKind::Action), ("h /", ButtonKind::Op), ("l *", ButtonKind::Op)],
        [("7", ButtonKind::Num), ("8", ButtonKind::Num), ("9", ButtonKind::Num), ("k -", ButtonKind::Op)],
        [("4", ButtonKind::Num), ("5", ButtonKind::Num), ("6", ButtonKind::Num), ("j +", ButtonKind::Op)],
        [("1", ButtonKind::Num), ("2", ButtonKind::Num), ("3", ButtonKind::Num), ("=  ", ButtonKind::Equals)],
        [("0", ButtonKind::Num), ("0", ButtonKind::Num), (".", ButtonKind::Num), ("=  ", ButtonKind::Equals)],
    ];

    let start_x = x + 12;
    for r in 0..rows {
        for c in 0..cols {
            let (text, kind) = buttons[r as usize][c as usize];
            // Skip duplicate 0 / =
            if r == 4 && c == 1 { continue; }
            if r == 4 && c == 3 { continue; }

            let mut cell_x = start_x + c * (bw + gap);
            let mut cell_w = bw;
            if r == 4 && c == 0 { cell_w = bw * 2 + gap; }
            if r == 3 && c == 3 { /* tall */ }

            let cell_y = grid_y + r * (bh + gap);
            let mut cell_h = bh;
            if r == 3 && c == 3 { cell_h = bh * 2 + gap; }

            let (top, bot, fg) = match kind {
                ButtonKind::Num => (Color::rgb(0x33, 0x3d, 0x52), Color::rgb(0x1e, 0x26, 0x36), Color::WHITE),
                ButtonKind::Op => (Color::rgb(0xff, 0x9f, 0x0a), Color::rgb(0xc6, 0x6f, 0x00), Color::WHITE),
                ButtonKind::Action => (Color::rgb(0x60, 0x6a, 0x80), Color::rgb(0x40, 0x48, 0x5a), Color::WHITE),
                ButtonKind::Equals => (Color::rgb(0x66, 0xbb, 0x6a), Color::rgb(0x2e, 0x7d, 0x32), Color::WHITE),
            };

            // Rangli gradient tugma
            draw_button(fb, cell_x, cell_y, cell_w, cell_h, top, bot);

            // Matn markazlashtirilgan
            let tw = (text.len() as u32) * 8 * 2;
            fb.draw_text(
                cell_x + (cell_w.saturating_sub(tw)) / 2,
                cell_y + (cell_h.saturating_sub(16)) / 2,
                text,
                fg,
                2,
            );
        }
    }
}

#[derive(Clone, Copy)]
enum ButtonKind { Num, Op, Action, Equals }

fn draw_button(fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32, top: Color, bot: Color) {
    fb.shadow_rect(x, y + 2, w, h, Color::rgb(0x00, 0x00, 0x06));
    // Vertikal gradient yumaloq burchaklar bilan
    fb.round_rect(x, y, w, h, 8, top);
    // Pastki yarmiga bot rangiga gradient
    fb.vgradient(x + 2, y + h / 2, w - 4, h / 2 - 2, top, bot);
}

//! Calculator app — to'liq ishlovchi tugmalar gridi bilan.

use alloc::format;
use alloc::string::{String, ToString};

use crate::graphics::framebuffer::{Color, FontSize, Framebuffer};
use crate::ui::Shell;

#[derive(Clone, Copy)]
enum Op { Add, Sub, Mul, Div }

impl Op {
    fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b, Op::Sub => a - b, Op::Mul => a * b,
            Op::Div => if b == 0.0 { 0.0 } else { a / b },
        }
    }
    fn label(self) -> &'static str {
        match self { Op::Add => "+", Op::Sub => "-", Op::Mul => "x", Op::Div => "/" }
    }
}

pub struct CalcState {
    display: String,
    accum: f64,
    pending: Option<Op>,
    fresh: bool,
}

impl CalcState {
    pub fn new() -> Self {
        Self { display: String::from("0"), accum: 0.0, pending: None, fresh: true }
    }
    pub fn handle_key(&mut self, code: u16) -> bool {
        let digit = match code {
            2 => Some('1'), 3 => Some('2'), 4 => Some('3'),
            5 => Some('4'), 6 => Some('5'), 7 => Some('6'),
            8 => Some('7'), 9 => Some('8'), 10 => Some('9'),
            11 => Some('0'),
            _ => None,
        };
        if let Some(d) = digit {
            if self.fresh { self.display.clear(); self.fresh = false; }
            if self.display == "0" { self.display.clear(); }
            self.display.push(d);
            return true;
        }
        let op = match code {
            36 => Some(Op::Add), 37 => Some(Op::Sub),
            38 => Some(Op::Mul), 35 => Some(Op::Div),
            _ => None,
        };
        if let Some(o) = op {
            self.compute_pending();
            self.pending = Some(o);
            self.fresh = true;
            return true;
        }
        if code == 28 { self.compute_pending(); self.pending = None; self.fresh = true; return true; }
        if code == 14 {
            if self.fresh { self.display = String::from("0"); }
            else {
                self.display.pop();
                if self.display.is_empty() { self.display.push('0'); self.fresh = true; }
            }
            return true;
        }
        if code == 46 {
            self.display = String::from("0"); self.accum = 0.0;
            self.pending = None; self.fresh = true;
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
        } else { self.accum = cur; }
    }
}

fn format_num(n: f64) -> String {
    if (n - (n as i64 as f64)).abs() < 1e-9 {
        format!("{}", n as i64)
    } else {
        let scaled = (n * 10000.0 + if n >= 0.0 { 0.5 } else { -0.5 }) as i64;
        let int = scaled / 10000;
        let frac = (scaled.abs() % 10000) as u32;
        let s = format!("{}.{:04}", int, frac);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

pub fn draw(shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32) {
    // Display panel
    let disp_y = y + 12;
    let disp_h = 80u32;
    fb.round_rect(x + 16, disp_y, w - 32, disp_h, 12, Color::rgb(0x05, 0x0a, 0x16));

    // Pending op indicator (chap)
    if let Some(op) = shell.calculator.pending {
        fb.draw_text_aa(x + 30, disp_y + 12, op.label(), Color::ACCENT, FontSize::Px24, true);
    }

    // Display
    let display = &shell.calculator.display;
    let size = if display.len() <= 8 {
        FontSize::Px32
    } else if display.len() <= 14 {
        FontSize::Px24
    } else {
        FontSize::Px20
    };
    let dw = Framebuffer::measure_text_aa(display, size, true);
    fb.draw_text_aa(
        x + w - dw - 28,
        disp_y + (disp_h - size.height()) / 2,
        display,
        Color::WHITE,
        size,
        true,
    );

    // Tugmalar gridi 4x5
    let grid_y = disp_y + disp_h + 14;
    let grid_h = (y + h).saturating_sub(grid_y).saturating_sub(8);
    let cols = 4u32;
    let rows = 5u32;
    let gap = 8u32;
    let avail_w = w - 32;
    let avail_h = grid_h.saturating_sub(8);
    let bw = (avail_w - (cols - 1) * gap) / cols;
    let bh = (avail_h - (rows - 1) * gap) / rows;

    let buttons: [[(&str, ButtonKind); 4]; 5] = [
        [("C", ButtonKind::Action), ("<-", ButtonKind::Action), ("h /", ButtonKind::Op), ("l x", ButtonKind::Op)],
        [("7", ButtonKind::Num), ("8", ButtonKind::Num), ("9", ButtonKind::Num), ("k -", ButtonKind::Op)],
        [("4", ButtonKind::Num), ("5", ButtonKind::Num), ("6", ButtonKind::Num), ("j +", ButtonKind::Op)],
        [("1", ButtonKind::Num), ("2", ButtonKind::Num), ("3", ButtonKind::Num), ("=", ButtonKind::Equals)],
        [("0", ButtonKind::Num), ("0", ButtonKind::Num), (".", ButtonKind::Num), ("=", ButtonKind::Equals)],
    ];

    let start_x = x + 16;
    for r in 0..rows {
        for c in 0..cols {
            if r == 4 && c == 1 { continue; }
            if r == 4 && c == 3 { continue; }

            let (text, kind) = buttons[r as usize][c as usize];
            let cell_x = start_x + c * (bw + gap);
            let mut cell_w = bw;
            if r == 4 && c == 0 { cell_w = bw * 2 + gap; }

            let cell_y = grid_y + r * (bh + gap);
            let mut cell_h = bh;
            if r == 3 && c == 3 { cell_h = bh * 2 + gap; }

            let (top, bot) = match kind {
                ButtonKind::Num => (Color::rgb(0x37, 0x3f, 0x55), Color::rgb(0x1f, 0x25, 0x35)),
                ButtonKind::Op => (Color::rgb(0xff, 0x9f, 0x0a), Color::rgb(0xc6, 0x6f, 0x00)),
                ButtonKind::Action => (Color::rgb(0x6c, 0x77, 0x8d), Color::rgb(0x44, 0x4d, 0x60)),
                ButtonKind::Equals => (Color::rgb(0x66, 0xbb, 0x6a), Color::rgb(0x2e, 0x7d, 0x32)),
            };
            draw_button(fb, cell_x, cell_y, cell_w, cell_h, top, bot);

            // AA matn
            let ts = if cell_h > 50 { FontSize::Px24 } else { FontSize::Px20 };
            let tw = Framebuffer::measure_text_aa(text, ts, true);
            fb.draw_text_aa(
                cell_x + (cell_w.saturating_sub(tw)) / 2,
                cell_y + (cell_h.saturating_sub(ts.height())) / 2,
                text, Color::WHITE, ts, true,
            );
        }
    }
}

#[derive(Clone, Copy)]
enum ButtonKind { Num, Op, Action, Equals }

fn draw_button(fb: &mut Framebuffer, x: u32, y: u32, w: u32, h: u32, top: Color, bot: Color) {
    fb.shadow_rect(x, y + 2, w, h, Color::rgb(0x00, 0x00, 0x06));
    fb.round_rect(x, y, w, h, 10, top);
    fb.vgradient(x + 2, y + h / 2, w - 4, h / 2 - 2, top, bot);
}

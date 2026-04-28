//! Welcome ekran — boot status va branding.

use crate::graphics::framebuffer::{Color, Framebuffer};
use crate::ui::Shell;

pub fn draw(_shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, _h: u32) {
    // Compact / large layoutni width ga qarab tanlaymiz.
    let big = w >= 500;

    let title_scale = if big { 6 } else { 3 };
    let center_x = x + w / 2;
    let title_w = 7 * 8 * title_scale;
    fb.draw_text(
        center_x.saturating_sub(title_w / 2),
        y + 16,
        "ZaminOS",
        Color::ZAMIN_FG,
        title_scale,
    );

    let sub = "Konvergent OS for aarch64";
    let sub_scale = if big { 2 } else { 1 };
    let sub_w = (sub.len() as u32) * 8 * sub_scale;
    fb.draw_text(
        center_x.saturating_sub(sub_w / 2),
        y + if big { 100 } else { 60 },
        sub,
        Color::WHITE,
        sub_scale,
    );

    let tag = "Mobile-first, desktop-ready";
    let tag_w = (tag.len() as u32) * 8;
    fb.draw_text(
        center_x.saturating_sub(tag_w / 2),
        y + if big { 130 } else { 80 },
        tag,
        Color::ACCENT,
        1,
    );

    let lines = [
        "[ OK ]  EL2 -> EL1 boot",
        "[ OK ]  MMU + 4 MiB heap",
        "[ OK ]  GICv2 + timer",
        "[ OK ]  Scheduler",
        "[ OK ]  ramfb framebuffer",
        "[ OK ]  virtio-input",
        "[ OK ]  Multi-app shell",
        "[ OK ]  Mobile/Desktop layout",
    ];

    let lh = if big { 22 } else { 14 };
    let line_scale = if big { 2 } else { 1 };
    let mut ly = y + if big { 170 } else { 110 };
    let lx = x + if big { 30 } else { 12 };
    for line in lines.iter() {
        fb.draw_text(lx, ly, line, Color::SUCCESS, line_scale);
        ly += lh;
    }
}

//! Welcome ekran — boot status va branding.

use crate::graphics::framebuffer::{Color, FontSize, Framebuffer};
use crate::ui::Shell;

pub fn draw(_shell: &Shell, fb: &mut Framebuffer, x: u32, y: u32, w: u32, _h: u32) {
    let big = w >= 500;

    let cx = x + w / 2;
    let title_size = if big { FontSize::Px32 } else { FontSize::Px24 };
    fb.draw_text_aa_centered(cx, y + 24, "ZaminOS", Color::ZAMIN_FG, title_size, true);

    let sub_size = if big { FontSize::Px20 } else { FontSize::Px16 };
    fb.draw_text_aa_centered(cx, y + 70, "Konvergent OS uchun aarch64", Color::WHITE, sub_size, false);
    fb.draw_text_aa_centered(cx, y + 96, "Mobile-first, desktop-ready", Color::ACCENT, FontSize::Px16, false);

    let lines = [
        "EL2 -> EL1 boot",
        "MMU + 4 MiB heap",
        "GICv2 + generic timer",
        "Kooperativ scheduler",
        "ramfb 800x600 framebuffer",
        "virtio-input keyboard/mouse",
        "Multi-app shell, 10 apps",
        "Mobile / Desktop konvergensiya",
    ];

    let lh = if big { 26 } else { 20 };
    let lf = if big { FontSize::Px20 } else { FontSize::Px16 };
    let mut ly = y + 140;
    let lx = x + if big { 50 } else { 16 };
    for line in lines.iter() {
        // Yashil tasdiq belgi
        fb.fill_circle((lx + 8) as i32, (ly + 10) as i32, 7, Color::SUCCESS);
        fb.draw_text_aa(lx + 4, ly + 2, "v", Color::WHITE, FontSize::Px16, true);
        fb.draw_text_aa(lx + 30, ly + 2, line, Color::WHITE, lf, false);
        ly += lh;
    }
}

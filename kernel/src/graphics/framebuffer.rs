//! Framebuffer abstraktsiyasi — chizish va matn render.
//!
//! Piksel formati XRGB8888 (LE u32): 0x00RRGGBB.

use font8x8::UnicodeFonts;
use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight, RasterizedChar};

/// Antialiased font o'lchami.
#[derive(Clone, Copy)]
pub enum FontSize {
    Px16,
    Px20,
    Px24,
    Px32,
}

impl FontSize {
    fn raster_height(self) -> RasterHeight {
        match self {
            FontSize::Px16 => RasterHeight::Size16,
            FontSize::Px20 => RasterHeight::Size20,
            FontSize::Px24 => RasterHeight::Size24,
            FontSize::Px32 => RasterHeight::Size32,
        }
    }
    pub fn height(self) -> u32 {
        match self {
            FontSize::Px16 => 16,
            FontSize::Px20 => 20,
            FontSize::Px24 => 24,
            FontSize::Px32 => 32,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Color(pub u32);

impl Color {
    pub const BLACK: Color = Color::rgb(0x10, 0x12, 0x18);
    pub const WHITE: Color = Color::rgb(0xff, 0xff, 0xff);
    pub const ZAMIN_BG: Color = Color::rgb(0x0a, 0x18, 0x2e); // chuqur ko'k
    pub const ZAMIN_FG: Color = Color::rgb(0xfd, 0xc4, 0x35); // tilla rangli sariq
    pub const ACCENT: Color = Color::rgb(0x29, 0xb6, 0xf6);   // havo rangli
    pub const SUCCESS: Color = Color::rgb(0x66, 0xbb, 0x6a);  // yashil
    pub const MUTED: Color = Color::rgb(0x90, 0xa4, 0xae);    // kulrang

    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }
}

pub struct Framebuffer {
    pixels: &'static mut [u32],
    width: u32,
    height: u32,
}

impl Framebuffer {
    pub fn new(pixels: &'static mut [u32], width: u32, height: u32) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn put(&mut self, x: u32, y: u32, c: Color) {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = c.0;
        }
    }

    #[inline]
    pub fn raw_pixel(&self, idx: usize) -> u32 {
        self.pixels[idx]
    }

    #[inline]
    pub fn set_raw_pixel(&mut self, idx: usize, v: u32) {
        self.pixels[idx] = v;
    }

    pub fn clear(&mut self, c: Color) {
        for p in self.pixels.iter_mut() {
            *p = c.0;
        }
    }

    pub fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, c: Color) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        for j in y..y_end {
            for i in x..x_end {
                self.pixels[(j * self.width + i) as usize] = c.0;
            }
        }
    }

    pub fn hline(&mut self, x: u32, y: u32, w: u32, c: Color) {
        self.fill_rect(x, y, w, 1, c);
    }

    pub fn vline(&mut self, x: u32, y: u32, h: u32, c: Color) {
        self.fill_rect(x, y, 1, h, c);
    }

    /// 8x8 pikselli ASCII belgini chizish. `scale` = 1 yoki 2.
    pub fn draw_char(&mut self, x: u32, y: u32, ch: char, fg: Color, scale: u32) {
        let glyph = match font8x8::BASIC_FONTS.get(ch) {
            Some(g) => g,
            None => return,
        };
        for (row, byte) in glyph.iter().enumerate() {
            for col in 0..8u32 {
                if byte & (1 << col) != 0 {
                    let px = x + col * scale;
                    let py = y + (row as u32) * scale;
                    if scale == 1 {
                        self.put(px, py, fg);
                    } else {
                        self.fill_rect(px, py, scale, scale, fg);
                    }
                }
            }
        }
    }

    /// Matn satrini chizish.
    pub fn draw_text(&mut self, mut x: u32, y: u32, s: &str, fg: Color, scale: u32) {
        for ch in s.chars() {
            self.draw_char(x, y, ch, fg, scale);
            x += 8 * scale;
        }
    }

    /// Markazlashtirilgan matn (X bo'yicha).
    pub fn draw_text_centered(&mut self, y: u32, s: &str, fg: Color, scale: u32) {
        let text_w = (s.chars().count() as u32) * 8 * scale;
        let x = self.width.saturating_sub(text_w) / 2;
        self.draw_text(x, y, s, fg, scale);
    }

    /// Antialiased matn — Noto Sans Mono cratesidan grayscale glyphlarni
    /// joriy fonga blend qilish orqali. Katta va smooth ko'rinadi.
    pub fn draw_text_aa(&mut self, x: u32, y: u32, s: &str, fg: Color, size: FontSize, bold: bool) -> u32 {
        let weight = if bold { FontWeight::Bold } else { FontWeight::Regular };
        let mut cur_x = x;
        for ch in s.chars() {
            if let Some(glyph) = get_raster(ch, weight, size.raster_height()) {
                self.blit_glyph(cur_x, y, &glyph, fg);
                cur_x += glyph.width() as u32;
            } else {
                cur_x += size.height() / 2;
            }
        }
        cur_x - x
    }

    /// Antialiased matn kengligini hisoblash (chizmasdan).
    pub fn measure_text_aa(s: &str, size: FontSize, bold: bool) -> u32 {
        let weight = if bold { FontWeight::Bold } else { FontWeight::Regular };
        let mut w = 0u32;
        for ch in s.chars() {
            if let Some(glyph) = get_raster(ch, weight, size.raster_height()) {
                w += glyph.width() as u32;
            } else {
                w += size.height() / 2;
            }
        }
        w
    }

    /// Markazlashtirilgan AA matn.
    pub fn draw_text_aa_centered(&mut self, cx: u32, y: u32, s: &str, fg: Color, size: FontSize, bold: bool) {
        let w = Self::measure_text_aa(s, size, bold);
        let x = cx.saturating_sub(w / 2);
        self.draw_text_aa(x, y, s, fg, size, bold);
    }

    fn blit_glyph(&mut self, x: u32, y: u32, glyph: &RasterizedChar, fg: Color) {
        let (fr, fg_, fb) = ((fg.0 >> 16) & 0xff, (fg.0 >> 8) & 0xff, fg.0 & 0xff);
        for (j, row) in glyph.raster().iter().enumerate() {
            for (i, intensity) in row.iter().enumerate() {
                let intensity = *intensity as u32;
                if intensity == 0 {
                    continue;
                }
                let px = x + i as u32;
                let py = y + j as u32;
                if px >= self.width || py >= self.height {
                    continue;
                }
                let idx = (py * self.width + px) as usize;
                let bg = self.pixels[idx];
                let (br, bg_, bb) = ((bg >> 16) & 0xff, (bg >> 8) & 0xff, bg & 0xff);
                let inv = 255 - intensity;
                let r = (br * inv + fr * intensity) / 255;
                let g = (bg_ * inv + fg_ * intensity) / 255;
                let b = (bb * inv + fb * intensity) / 255;
                self.pixels[idx] = (r << 16) | (g << 8) | b;
            }
        }
    }

    /// Vertikal gradiyent — yuqoridan pastga ranglar interpolatsiyasi.
    pub fn vgradient(&mut self, x: u32, y: u32, w: u32, h: u32, top: Color, bottom: Color) {
        if h == 0 {
            return;
        }
        let (tr, tg, tb) = ((top.0 >> 16) & 0xff, (top.0 >> 8) & 0xff, top.0 & 0xff);
        let (br, bg, bb) = ((bottom.0 >> 16) & 0xff, (bottom.0 >> 8) & 0xff, bottom.0 & 0xff);
        for j in 0..h {
            let t = j as i32;
            let total = (h - 1).max(1) as i32;
            let r = (tr as i32 + (br as i32 - tr as i32) * t / total) as u32;
            let g = (tg as i32 + (bg as i32 - tg as i32) * t / total) as u32;
            let b = (tb as i32 + (bb as i32 - tb as i32) * t / total) as u32;
            let c = Color((r << 16) | (g << 8) | b);
            self.fill_rect(x, y + j, w, 1, c);
        }
    }

    /// Yumaloq burchakli to'rtburchak (taxminiy — burchaklarni 4 piksel kvadratdan kesadi).
    pub fn round_rect(&mut self, x: u32, y: u32, w: u32, h: u32, r: u32, c: Color) {
        if w <= 2 * r || h <= 2 * r {
            self.fill_rect(x, y, w, h, c);
            return;
        }
        // Asosiy uch qism: yuqori, markaz, pastki
        self.fill_rect(x + r, y, w - 2 * r, h, c);
        self.fill_rect(x, y + r, w, h - 2 * r, c);
        // Burchaklar — disk to'ldirish
        for cy_off in 0..r {
            for cx_off in 0..r {
                let dx = (r - cx_off) as i32;
                let dy = (r - cy_off) as i32;
                if dx * dx + dy * dy <= (r as i32) * (r as i32) {
                    self.put(x + cx_off, y + cy_off, c);
                    self.put(x + w - 1 - cx_off, y + cy_off, c);
                    self.put(x + cx_off, y + h - 1 - cy_off, c);
                    self.put(x + w - 1 - cx_off, y + h - 1 - cy_off, c);
                }
            }
        }
    }

    /// Statvalar / shadow uchun yarim shaffof emulyatsiya — ko'p marta darken qilish.
    /// Real alpha-blend yo'q, lekin chuqurroq ko'k ranggа qoplaydi.
    pub fn shadow_rect(&mut self, x: u32, y: u32, w: u32, h: u32, alpha_dark: Color) {
        // Soya — pastga va o'ngga 4 piksel ofsetda yarim aralash rang.
        self.fill_rect(x + 4, y + 4, w, h, alpha_dark);
    }

    /// To'la disk (radius bo'yicha).
    pub fn fill_circle(&mut self, cx: i32, cy: i32, r: u32, c: Color) {
        let r2 = (r as i32) * (r as i32);
        for j in -(r as i32)..=(r as i32) {
            for i in -(r as i32)..=(r as i32) {
                if i * i + j * j <= r2 {
                    let px = cx + i;
                    let py = cy + j;
                    if px >= 0 && (px as u32) < self.width && py >= 0 && (py as u32) < self.height {
                        self.put(px as u32, py as u32, c);
                    }
                }
            }
        }
    }
}

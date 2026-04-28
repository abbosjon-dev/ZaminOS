//! Framebuffer abstraktsiyasi — chizish va matn render.
//!
//! Piksel formati XRGB8888 (LE u32): 0x00RRGGBB.

use font8x8::UnicodeFonts;

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
}

//! QEMU `ramfb` qurilmasi — RAM da joylashgan framebuffer.
//!
//! Ishlash printsipi:
//!   1. fw_cfg dan "etc/ramfb" entry'ni topamiz
//!   2. RamFbCfg strukturani (28 bayt, big-endian) DMA bilan yuboramiz
//!   3. QEMU keyin shu manzildan piksellarni o'qib displayga chiqaradi
//!
//! Format: XRGB8888 (DRM_FORMAT_XRGB8888 = 0x34325258) — har piksel 4 bayt:
//!   [B][G][R][X]  little-endian holda u32 sifatida.

use crate::drivers::fw_cfg;

pub const FB_WIDTH: u32 = 800;
pub const FB_HEIGHT: u32 = 600;
pub const FB_BPP: u32 = 4;
pub const FB_STRIDE: u32 = FB_WIDTH * FB_BPP;
pub const FB_SIZE: usize = (FB_STRIDE * FB_HEIGHT) as usize;

/// DRM_FORMAT_XRGB8888 — bo'sh, R, G, B (low->high).
const DRM_FORMAT_XRGB8888: u32 = 0x34325258;

#[repr(C, align(4096))]
struct Framebuffer {
    pixels: [u32; (FB_WIDTH * FB_HEIGHT) as usize],
}

static mut FB: Framebuffer = Framebuffer {
    pixels: [0; (FB_WIDTH * FB_HEIGHT) as usize],
};

// ramfb config strukturasi 28 bayt (Linux source: drivers/firmware/qemu_fw_cfg.c).
// Rust struct'idagi u64 alignment 32 baytga ko'paytiradi, shuning uchun
// qo'lda [u8; 28] qurib qo'yamiz, hamma maydonlar big-endian.
const RAMFB_CFG_SIZE: usize = 28;

fn build_cfg(addr: u64, w: u32, h: u32, stride: u32) -> [u8; RAMFB_CFG_SIZE] {
    let mut buf = [0u8; RAMFB_CFG_SIZE];
    buf[0..8].copy_from_slice(&addr.to_be_bytes());
    buf[8..12].copy_from_slice(&DRM_FORMAT_XRGB8888.to_be_bytes());
    buf[12..16].copy_from_slice(&0u32.to_be_bytes()); // flags
    buf[16..20].copy_from_slice(&w.to_be_bytes());
    buf[20..24].copy_from_slice(&h.to_be_bytes());
    buf[24..28].copy_from_slice(&stride.to_be_bytes());
    buf
}

/// ramfb ni QEMU bilan ulashga urin. Muvaffaqiyatli bo'lsa Some(buffer ptr).
pub fn init() -> Option<&'static mut [u32]> {
    let entry = fw_cfg::find_file("etc/ramfb")?;

    let fb_addr = unsafe { core::ptr::addr_of_mut!(FB.pixels) as u64 };
    let cfg = build_cfg(fb_addr, FB_WIDTH, FB_HEIGHT, FB_STRIDE);

    match fw_cfg::write_to(entry.select, &cfg) {
        Ok(()) => {}
        Err(e) => {
            crate::println!("[ramfb] write_to xatoligi: 0x{:08x}", e);
            return None;
        }
    }

    // SAFETY: bizning static FB, faqat shu yerdan oshkor qilinadi.
    let pixels = unsafe {
        core::slice::from_raw_parts_mut(
            core::ptr::addr_of_mut!(FB.pixels) as *mut u32,
            (FB_WIDTH * FB_HEIGHT) as usize,
        )
    };
    Some(pixels)
}

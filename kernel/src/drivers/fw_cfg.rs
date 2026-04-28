//! QEMU fw_cfg interfeysi (`virt` mashinasida 0x09020000).
//!
//! Faqat ramfb sozlash uchun zarur bo'lgan funksiyalar:
//!   - File directory (selector 0x19) ni o'qish va "etc/ramfb" entry'ni topish
//!   - Topilgan selectorga DMA orqali yozish
//!
//! Eslatma: fw_cfg registrlari aarch64 virt da **big-endian** sifatida talqin
//! qilinadi, shuning uchun u16/u32/u64 qiymatlarni `to_be()` bilan svap qilamiz.

use core::ptr::{read_volatile, write_volatile};

const FW_CFG_BASE: usize = 0x0902_0000;
const FW_CFG_DATA: usize = FW_CFG_BASE + 0x00;
const FW_CFG_SEL: usize = FW_CFG_BASE + 0x08;
const FW_CFG_DMA: usize = FW_CFG_BASE + 0x10;

const SEL_FILE_DIR: u16 = 0x19;

const DMA_CTL_ERROR: u32 = 0x01;
const DMA_CTL_READ: u32 = 0x02;
const DMA_CTL_SKIP: u32 = 0x04;
const DMA_CTL_SELECT: u32 = 0x08;
const DMA_CTL_WRITE: u32 = 0x10;

#[repr(C, align(8))]
struct DmaAccess {
    control: u32, // big-endian on the wire
    length: u32,  // big-endian
    address: u64, // big-endian
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FwCfgFile {
    pub size: u32,        // big-endian on the wire
    pub select: u16,      // big-endian
    pub _reserved: u16,
    pub name: [u8; 56],   // ASCII, null-terminated
}

/// Selector ni o'rnatish (plain MMIO, big-endian write).
unsafe fn select(sel: u16) {
    write_volatile(FW_CFG_SEL as *mut u16, sel.to_be());
}

/// Plain MMIO orqali ketma-ket bayt o'qish (joriy selector dan).
unsafe fn read_byte() -> u8 {
    read_volatile(FW_CFG_DATA as *const u8)
}

/// `n` bayt o'qib `dst` ga yozadi.
unsafe fn read_bytes(dst: &mut [u8]) {
    for b in dst.iter_mut() {
        *b = read_byte();
    }
}

/// DMA orqali yozish: joriy buffer ni belgilangan selectorga jo'natadi.
unsafe fn dma_op(control: u32, length: u32, addr: u64) -> Result<(), u32> {
    let access = DmaAccess {
        control: control.to_be(),
        length: length.to_be(),
        address: addr.to_be(),
    };
    let access_ptr = &access as *const DmaAccess as u64;

    // DMA register ham 64-bit big-endian.
    write_volatile(FW_CFG_DMA as *mut u64, access_ptr.to_be());

    // Tugashini kutish: control bitlari tozalanadi (yoki ERROR yoqiladi).
    let mut spins = 0u32;
    loop {
        let cur = u32::from_be(read_volatile(&access.control as *const u32));
        if cur & DMA_CTL_ERROR != 0 {
            return Err(cur);
        }
        if cur == 0 {
            return Ok(());
        }
        spins += 1;
        if spins > 1_000_000 {
            return Err(cur);
        }
        core::hint::spin_loop();
    }
}

/// FILE_DIR ni skanerlab `target_name` ga teng entry topish.
pub fn find_file(target_name: &str) -> Option<FwCfgFile> {
    unsafe {
        select(SEL_FILE_DIR);

        let mut count_buf = [0u8; 4];
        read_bytes(&mut count_buf);
        let count = u32::from_be_bytes(count_buf);

        for _ in 0..count {
            let mut entry = FwCfgFile {
                size: 0,
                select: 0,
                _reserved: 0,
                name: [0; 56],
            };

            let mut tmp4 = [0u8; 4];
            read_bytes(&mut tmp4);
            entry.size = u32::from_be_bytes(tmp4);

            let mut tmp2 = [0u8; 2];
            read_bytes(&mut tmp2);
            entry.select = u16::from_be_bytes(tmp2);

            read_bytes(&mut tmp2); // reserved

            read_bytes(&mut entry.name);

            // Nomi ASCII, null-terminated. Solishtiramiz.
            let name_len = entry.name.iter().position(|&b| b == 0).unwrap_or(56);
            if &entry.name[..name_len] == target_name.as_bytes() {
                return Some(entry);
            }
        }
        None
    }
}

/// Belgilangan selector ga `data` ni DMA bilan yozadi.
pub fn write_to(selector: u16, data: &[u8]) -> Result<(), u32> {
    unsafe {
        let ctl = DMA_CTL_SELECT | DMA_CTL_WRITE | ((selector as u32) << 16);
        dma_op(ctl, data.len() as u32, data.as_ptr() as u64)
    }
}

#[allow(dead_code)]
const _UNUSED: u32 = DMA_CTL_READ | DMA_CTL_SKIP;

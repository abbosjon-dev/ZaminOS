//! virtio-input drayveri.
//!
//! QEMU virt mashinasi virtio-mmio bus 0x0a000000 da, har slot 0x200 bayt,
//! jami 32 slot. Har bir `-device virtio-XXX-device` qatorga slot beriladi.
//! Bu yerda biz `virtio-keyboard-device` va `virtio-tablet-device` ni topib,
//! ulardan event'larni o'qiymiz.

use core::ptr::NonNull;

use virtio_drivers::device::input::VirtIOInput;
use virtio_drivers::transport::mmio::{MmioTransport, VirtIOHeader};
use virtio_drivers::transport::{DeviceType, Transport};

use crate::drivers::virtio::ZaminHal;

pub const VIRTIO_MMIO_BASE: usize = 0x0a00_0000;
pub const VIRTIO_MMIO_STRIDE: usize = 0x200;
pub const NUM_SLOTS: usize = 32;

pub type Input = VirtIOInput<ZaminHal, MmioTransport>;

/// Topilgan har bir virtio-input qurilma uchun bir `Input` instance qaytariladi.
/// Qurilmalar topilgan tartibda (slot bo'yicha o'sishda).
pub fn probe_inputs() -> alloc::vec::Vec<Input> {
    let mut found = alloc::vec::Vec::new();

    for slot in 0..NUM_SLOTS {
        let base = VIRTIO_MMIO_BASE + slot * VIRTIO_MMIO_STRIDE;
        let header_ptr = match NonNull::new(base as *mut VirtIOHeader) {
            Some(p) => p,
            None => continue,
        };

        let transport = match unsafe { MmioTransport::new(header_ptr) } {
            Ok(t) => t,
            Err(_) => continue,
        };

        if transport.device_type() != DeviceType::Input {
            continue;
        }

        match VirtIOInput::<ZaminHal, MmioTransport>::new(transport) {
            Ok(input) => {
                crate::println!(
                    "[virtio] slot {} ({:#x}): input device topildi",
                    slot,
                    base
                );
                found.push(input);
            }
            Err(e) => {
                crate::println!(
                    "[virtio] slot {} input init xatoligi: {:?}",
                    slot, e
                );
            }
        }
    }

    found
}

// --- Linux input event codes (subset) ---
// Manba: linux/input-event-codes.h

pub const EV_SYN: u16 = 0x00;
pub const EV_KEY: u16 = 0x01;
pub const EV_REL: u16 = 0x02;
pub const EV_ABS: u16 = 0x03;

pub const REL_X: u16 = 0x00;
pub const REL_Y: u16 = 0x01;

pub const ABS_X: u16 = 0x00;
pub const ABS_Y: u16 = 0x01;

/// Klaviatura scancode'idan ASCII belgini chiqaradi (lower-case, US layout).
/// Ko'pchilik virtio-keyboard'lar Linux KEY_* kodlari yuboradi.
pub fn keycode_to_char(code: u16) -> Option<char> {
    Some(match code {
        2 => '1',
        3 => '2',
        4 => '3',
        5 => '4',
        6 => '5',
        7 => '6',
        8 => '7',
        9 => '8',
        10 => '9',
        11 => '0',
        16 => 'q',
        17 => 'w',
        18 => 'e',
        19 => 'r',
        20 => 't',
        21 => 'y',
        22 => 'u',
        23 => 'i',
        24 => 'o',
        25 => 'p',
        30 => 'a',
        31 => 's',
        32 => 'd',
        33 => 'f',
        34 => 'g',
        35 => 'h',
        36 => 'j',
        37 => 'k',
        38 => 'l',
        44 => 'z',
        45 => 'x',
        46 => 'c',
        47 => 'v',
        48 => 'b',
        49 => 'n',
        50 => 'm',
        57 => ' ',
        14 => '\x08', // backspace
        28 => '\n',   // enter
        _ => return None,
    })
}

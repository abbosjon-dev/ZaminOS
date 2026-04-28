//! PL011 UART drayveri (QEMU `virt` mashinasi MMIO 0x09000000).
//!
//! Bu minimal versiya: QEMU PL011 ni allaqachon faollashtirgan holatda chiqaradi.
//! Haqiqiy apparatda baud rate va flag registrlar bilan ishlash kerak bo'ladi.

use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};

/// QEMU `virt` mashinasidagi PL011 bazaviy manzili.
const PL011_BASE: usize = 0x0900_0000;

const UART_DR: usize = 0x000; // Data register
const UART_FR: usize = 0x018; // Flag register
const FR_TXFF: u32 = 1 << 5; // Transmit FIFO full

pub struct Pl011Uart;

impl Pl011Uart {
    pub const fn new() -> Self {
        Self
    }

    fn write_byte(&self, byte: u8) {
        unsafe {
            // TX FIFO bo'shashini kut.
            while read_volatile((PL011_BASE + UART_FR) as *const u32) & FR_TXFF != 0 {}
            write_volatile((PL011_BASE + UART_DR) as *mut u32, byte as u32);
        }
    }
}

impl Write for Pl011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

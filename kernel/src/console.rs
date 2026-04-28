//! Global konsol — UART ustida `print!` / `println!` makroslari.
//!
//! Bu boshlang'ich versiya bir-yadroli: tashqi cratelar yo'q, oddiy `UnsafeCell`.
//! Faza 4 da SMP yoqilganda spinlock bilan almashtiriladi.

use core::cell::UnsafeCell;
use core::fmt::{self, Write};

use crate::drivers::uart_pl011::Pl011Uart;

struct GlobalConsole {
    uart: UnsafeCell<Pl011Uart>,
}

unsafe impl Sync for GlobalConsole {}

static CONSOLE: GlobalConsole = GlobalConsole {
    uart: UnsafeCell::new(Pl011Uart::new()),
};

pub fn _print(args: fmt::Arguments) {
    // SAFETY: bir-yadroli, prerivaniyalar hali yoqilmagan. Faza 3+ da bu o'zgaradi.
    unsafe {
        let uart = &mut *CONSOLE.uart.get();
        let _ = uart.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

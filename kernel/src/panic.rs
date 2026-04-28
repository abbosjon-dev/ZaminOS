use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crate::println!();
    crate::println!("[ZaminOS] *** KERNEL PANIC ***");
    crate::println!("{}", info);
    loop {
        unsafe { core::arch::asm!("wfe") };
    }
}

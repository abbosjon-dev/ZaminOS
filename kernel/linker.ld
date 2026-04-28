ENTRY(_start)

/*
 * QEMU `virt` machine yuklash manzili: 0x40000000.
 * `-kernel` flag bilan QEMU kernelni shu manzilga joylaydi va PC ni unga o'rnatadi.
 */
KERNEL_LOAD_ADDR = 0x40000000;

SECTIONS
{
    . = KERNEL_LOAD_ADDR;
    __kernel_start = .;

    .text : ALIGN(4K) {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss : ALIGN(4K) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(16);
        __bss_end = .;
    }

    . = ALIGN(4K);
    __kernel_end = .;
    /* Linux ARM64 image header `image_size` maydoni uchun absolyut qiymat. */
    __image_size = __kernel_end - __kernel_start;

    /DISCARD/ : {
        *(.comment)
        *(.note .note.*)
        *(.eh_frame .eh_frame_hdr)
    }
}

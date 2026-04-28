// ZaminOS aarch64 boot stub.
// QEMU `-kernel` bilan kernel 0x40000000 ga yuklanadi va boshqaruv _start ga beriladi.
// Vazifa: faqat birlamchi yadro davom etsin, BSS ni tozala, stack o'rnat, Rust ga sakra.

.section .text.boot, "ax"
.global _start

// ARM64 Linux kernel image header (Documentation/arm64/booting.rst).
// Bu sarlavha QEMU va UTM ga binar Linux-uslubidagi aarch64 yadro
// ekanligini bildiradi. Sarlavhasiz UTM SE "Linux Kernel" rejimida tanimaydi.
_start:
    b       primary_entry        // code0:  initial branch
    .long   0                    // code1:  reserved
    .quad   0                    // text_offset (yuklash manzilidan ofset)
    .quad   __image_size         // image_size (linker hisoblaydi)
    .quad   8                    // flags: LE + 4 KiB sahifalar
    .quad   0                    // reserved
    .quad   0                    // reserved
    .quad   0                    // reserved
    .ascii  "ARM\x64"            // magic: "ARM\x64"
    .long   0                    // PE header offset (yo'q)

primary_entry:
    // MPIDR_EL1[7:0] = CPU ID. Faqat 0-yadro davom etadi, qolganlari park qiladi.
    mrs     x0, mpidr_el1
    and     x0, x0, #0xff
    cbnz    x0, .Lpark

    // BSS ni nolga to'ldir.
    adrp    x0, __bss_start
    add     x0, x0, :lo12:__bss_start
    adrp    x1, __bss_end
    add     x1, x1, :lo12:__bss_end
.Lbss_loop:
    cmp     x0, x1
    b.hs    .Lbss_done
    str     xzr, [x0], #8
    b       .Lbss_loop
.Lbss_done:

    // Stack ni kernel obrazidan past joyga (KERNEL_LOAD_ADDR ostiga) o'rnat.
    // Bu joy QEMU virt RAM diapazonida (RAM 0x40000000 dan boshlanadi),
    // shuning uchun stackni kernel ustiga: __kernel_end + 64KiB qo'yamiz.
    adrp    x0, __kernel_end
    add     x0, x0, :lo12:__kernel_end
    add     x0, x0, #0x10000      // +64 KiB
    mov     sp, x0

    bl      kernel_main

.Lpark:
    wfe
    b       .Lpark

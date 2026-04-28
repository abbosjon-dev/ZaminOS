// ZaminOS aarch64 boot stub.
// QEMU `-kernel` bilan kernel 0x40000000 ga yuklanadi va boshqaruv _start ga beriladi.
// Vazifalar:
//   1. ARM64 Linux image header (UTM SE / QEMU "Linux" boot uchun)
//   2. Faqat birlamchi (CPU 0) yadro davom etsin
//   3. EL2 -> EL1 ga tushish (QEMU yadroni EL2 da boshlaydi)
//   4. BSS ni nolga to'ldirish
//   5. Stack o'rnatish, Rust ga sakrash

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
    // MPIDR_EL1[7:0] = CPU ID. Faqat 0-yadro davom etadi.
    mrs     x0, mpidr_el1
    and     x0, x0, #0xff
    cbnz    x0, .Lpark

    // Hozir EL2 da turibmiz (QEMU default). EL1 ga tushamiz.
    mrs     x0, CurrentEL
    lsr     x0, x0, #2
    cmp     x0, #2
    b.ne    .Lin_el1

    // EL2 -> EL1 sozlamalari:
    //   HCR_EL2.RW = 1 (EL1 aarch64)
    //   SPSR_EL2 = DAIF masked, EL1h
    //   ELR_EL2 = .Lin_el1
    mov     x0, #(1 << 31)       // RW=1
    msr     hcr_el2, x0

    // CNTHCTL_EL2: EL1 ga generic timer/counterga ruxsat (EL1PCEN, EL1PCTEN)
    mrs     x0, cnthctl_el2
    orr     x0, x0, #3
    msr     cnthctl_el2, x0
    msr     cntvoff_el2, xzr

    // SCTLR_EL1 ni xavfsiz boshlang'ich qiymatga
    mov     x0, #0x0800          // RES1 bitlari kelajakda
    movk    x0, #0x30d0, lsl #16
    msr     sctlr_el1, x0

    mov     x0, #0x3c5           // DAIF=1111, M=0101 (EL1h)
    msr     spsr_el2, x0
    adr     x0, .Lin_el1
    msr     elr_el2, x0
    eret

.Lin_el1:
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

    // Stack ni kernel obrazidan yuqori joyga (kernel ustiga 64 KiB) o'rnat.
    adrp    x0, __kernel_end
    add     x0, x0, :lo12:__kernel_end
    add     x0, x0, #0x10000      // +64 KiB
    mov     sp, x0

    bl      kernel_main

.Lpark:
    wfe
    b       .Lpark

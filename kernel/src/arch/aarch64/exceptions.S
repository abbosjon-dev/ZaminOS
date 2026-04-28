// AArch64 Exception Vector Table.
// VBAR_EL1 ga yuklash uchun 2048 bayt aligned, 16 ta entry × 128 bayt.
// Har bir entry: handler ga sakrash + qaytish.
//
// Entry layout (4 group × 4 entry):
//   Group 0 — Current EL with SP_EL0:    Sync, IRQ, FIQ, SError
//   Group 1 — Current EL with SP_ELx:    Sync, IRQ, FIQ, SError   <-- ishlatamiz
//   Group 2 — Lower EL (AArch64):        Sync, IRQ, FIQ, SError
//   Group 3 — Lower EL (AArch32):        Sync, IRQ, FIQ, SError

.macro SAVE_REGS
    sub     sp, sp, #256
    stp     x0,  x1,  [sp, #16 * 0]
    stp     x2,  x3,  [sp, #16 * 1]
    stp     x4,  x5,  [sp, #16 * 2]
    stp     x6,  x7,  [sp, #16 * 3]
    stp     x8,  x9,  [sp, #16 * 4]
    stp     x10, x11, [sp, #16 * 5]
    stp     x12, x13, [sp, #16 * 6]
    stp     x14, x15, [sp, #16 * 7]
    stp     x16, x17, [sp, #16 * 8]
    stp     x18, x19, [sp, #16 * 9]
    stp     x20, x21, [sp, #16 * 10]
    stp     x22, x23, [sp, #16 * 11]
    stp     x24, x25, [sp, #16 * 12]
    stp     x26, x27, [sp, #16 * 13]
    stp     x28, x29, [sp, #16 * 14]
    str     x30,      [sp, #16 * 15]
.endm

.macro RESTORE_REGS
    ldp     x0,  x1,  [sp, #16 * 0]
    ldp     x2,  x3,  [sp, #16 * 1]
    ldp     x4,  x5,  [sp, #16 * 2]
    ldp     x6,  x7,  [sp, #16 * 3]
    ldp     x8,  x9,  [sp, #16 * 4]
    ldp     x10, x11, [sp, #16 * 5]
    ldp     x12, x13, [sp, #16 * 6]
    ldp     x14, x15, [sp, #16 * 7]
    ldp     x16, x17, [sp, #16 * 8]
    ldp     x18, x19, [sp, #16 * 9]
    ldp     x20, x21, [sp, #16 * 10]
    ldp     x22, x23, [sp, #16 * 11]
    ldp     x24, x25, [sp, #16 * 12]
    ldp     x26, x27, [sp, #16 * 13]
    ldp     x28, x29, [sp, #16 * 14]
    ldr     x30,      [sp, #16 * 15]
    add     sp, sp, #256
.endm

.macro VECTOR_ENTRY label
.align 7
    b       \label
.endm

.section .text.exceptions, "ax"
.align 11                          // 2048-byte aligned (vector table aligment)
.global exception_vector_table
exception_vector_table:
    // Current EL with SP_EL0
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    // Current EL with SP_ELx  <-- biz bu yerdamiz
    VECTOR_ENTRY sync_current_elx
    VECTOR_ENTRY irq_current_elx
    VECTOR_ENTRY fiq_current_elx
    VECTOR_ENTRY serror_current_elx
    // Lower EL (AArch64)
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    // Lower EL (AArch32)
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler
    VECTOR_ENTRY default_handler

sync_current_elx:
    SAVE_REGS
    mov     x0, sp
    mrs     x1, esr_el1
    mrs     x2, elr_el1
    bl      handle_sync
    RESTORE_REGS
    eret

irq_current_elx:
    SAVE_REGS
    bl      handle_irq
    RESTORE_REGS
    eret

fiq_current_elx:
    SAVE_REGS
    bl      handle_fiq
    RESTORE_REGS
    eret

serror_current_elx:
    SAVE_REGS
    bl      handle_serror
    RESTORE_REGS
    eret

default_handler:
    SAVE_REGS
    bl      handle_unknown
    RESTORE_REGS
    eret

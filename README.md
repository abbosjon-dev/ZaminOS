# ZaminOS

ARM **aarch64** chiplariga moʻljallangan, **Rust** tilida yozilayotgan operatsion tizim. Asosiy maqsad — **mobile-first**, lekin tashqi monitor + klaviatura + sichqoncha ulanganda **desktop** rejimida ishlay oladigan **konvergent** OS.

> Holati: **Faza 0–1 yakunlandi.** Yadro QEMU `virt` mashinasida yuklanadi, PL011 UART orqali xabar chiqaradi.

## Yoʻl xaritasi

- [x] **Faza 0** — Toolchain, skelet, QEMU virt boot
- [x] **Faza 1** — PL011 UART, `println!` makros
- [ ] **Faza 2** — MMU, sahifalash, heap allokator
- [ ] **Faza 3** — GIC + generic timer (uzilishlar)
- [ ] **Faza 4** — Scheduler / async executor
- [ ] **Faza 5** — virtio-gpu framebuffer
- [ ] **Faza 6** — virtio-input (klaviatura, sichqoncha, touch)
- [ ] **Faza 7** — Raspberry Pi 4/5 portlash
- [ ] **Faza 8** — Userspace, ELF loader, syscalls
- [ ] **Faza 9** — Adaptiv shell va GUI (mobile ↔ desktop konvergensiyasi)

## Talablar

```bash
# Rust nightly + aarch64 bare-metal target
rustup toolchain install nightly --component rust-src llvm-tools-preview
rustup target add aarch64-unknown-none-softfloat --toolchain nightly
cargo install cargo-binutils

# Tizim paketlari (Ubuntu/Debian)
sudo apt install -y qemu-system-arm gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu
```

## Qurish va ishga tushirish

```bash
make build    # debug build
make run      # QEMU virt da ishga tushirish
make debug    # QEMU + gdb stub (port 1234)
make ios      # iPhone (UTM SE) uchun image tayyorlash
make clean
```

### iPhone'da sinab ko'rish (UTM SE)

```bash
make ios
# Natija: dist/zaminos-kernel.bin — UTM SE'ga yuklang
```

To'liq qo'llanma: [`dist/README-iOS.md`](dist/README-iOS.md)

Chiqishi:

```
==============================================
 ZaminOS v0.1.0  —  aarch64 Rust kernel
 Salom, dunyo! Yadro muvaffaqiyatli yuklandi.
==============================================

[boot] CPU: cortex-a72 (QEMU virt)
[boot] UART: PL011 @ 0x09000000
[boot] Faza 0 + 1 OK. Faza 2 (MMU) keyingi qadam.

[idle] Yadro idle tsiklga o'tdi (wfe).
```

QEMU dan chiqish: `Ctrl-A`, keyin `x`.

## Tuzilma

```
ZaminOS/
├── Cargo.toml              # workspace
├── rust-toolchain.toml     # nightly pin
├── Makefile                # build/run/debug
├── .cargo/config.toml      # target, rustflags, build-std
└── kernel/
    ├── Cargo.toml
    ├── linker.ld           # 0x40000000 (QEMU virt)
    └── src/
        ├── main.rs         # kernel_main
        ├── boot.S          # _start, BSS, stack
        ├── panic.rs
        ├── console.rs      # print!/println! makroslari
        └── drivers/
            └── uart_pl011.rs
```

## Litsenziya

LICENSE faylida koʻrsatilgan.

# ZaminOS

ARM **aarch64** chiplariga moʻljallangan, **Rust** tilida yozilayotgan operatsion tizim. Asosiy maqsad — **mobile-first**, lekin tashqi monitor + klaviatura + sichqoncha ulanganda **desktop** rejimida ishlay oladigan **konvergent** OS.

> Holati: **Faza 0–10 yakunlandi.** **Vector-style ikonalar** (uy, gear, palette, note, folder, clock procedural shaping bilan), **chiroyli wallpaper** (3-stop gradient + soft blob accent'lar, macOS Sonoma uslubli), **glassmorphism panellar** (yarim shaffof dock va widgetlar), **mobile lock screen** (katta soat + sana + notification card + qulfni ochish strelka). 10 ta app, AA fontlar, Windows 11-uslubli desktop, Android-uslubli mobile.

### Mobile (Android uslubli)

| Lock screen | Home (widgetlar) | App drawer (F2) | Music InApp |
|---|---|---|---|
| ![](dist/shot-12-mobile-lock.png) | ![](dist/shot-12b-mobile-home.png) | ![](dist/shot-13-mobile-drawer.png) | ![](dist/shot-14-mobile-music.png) |

### Desktop (Windows 11 / Linux uslubli)

| Welcome | Activity | Calculator |
|---|---|---|
| ![](dist/shot-01-desktop-welcome.png) | ![](dist/shot-02-desktop-activity.png) | ![](dist/shot-03-desktop-calculator.png) |
| **Keyboard** | **Terminal** | **Paint** |
| ![](dist/shot-04-desktop-keyboard.png) | ![](dist/shot-05-desktop-terminal.png) | ![](dist/shot-06-desktop-paint.png) |
| **Files** | **Music** | **Clock** |
| ![](dist/shot-07-desktop-files.png) | ![](dist/shot-08-desktop-music.png) | ![](dist/shot-09-desktop-clock.png) |
| **Settings** | **Start menu (F2)** | |
| ![](dist/shot-10-desktop-settings.png) | ![](dist/shot-11-desktop-start.png) | |


## Yoʻl xaritasi

- [x] **Faza 0** — Toolchain, skelet, QEMU virt boot
- [x] **Faza 1** — PL011 UART, `println!` makros
- [x] **Faza 2** — MMU (39-bit VA, 1 GiB blok identity map), 4 MiB heap allokator
- [x] **Faza 3** — Exception vector, GICv2, ARM generic timer (1 Hz)
- [x] **Faza 4** — Kooperativ scheduler (round-robin task'lar)
- [x] **Faza 5** — ramfb framebuffer (800×600 XRGB8888), 8×8 font matn rendering
- [x] **Faza 6** — virtio-input (klaviatura + tablet/sichqoncha), live event loop, dirty-flag redraw
- [x] **Faza 6.5** — Konvergent shell: top bar + side launcher + main content area, TAB bilan almashish
- [x] **Faza 7** — 6 ta app: Welcome, SysMon, Keyboard, Terminal, Paint, Clock. Mobile/Desktop layout. Gradient wallpaper, yumaloq panellar.
- [x] **Faza 8** — Rangli gradient app ikonalar, dock, traffic lights. 10 ta app.
- [x] **Faza 9** — Antialiased font, Windows 11 desktop, Android mobile.
- [x] **Faza 10** — **Vector ikonalar** (8x8 pattern emas — procedural shape: uy, bar chart, calculator, keyboard, terminal, palette, folder, music note, clock, gear). **Chiroyli wallpaper** (multi-stop gradient + 3 ta soft radial blob accent — macOS Sonoma uslubli). **Glassmorphism** yarim shaffof dock, widget panellar, search bar, notification cards. **Mobile lock screen** (katta soat, sana, notification card, qulfni ochish ko'rsatma). `put_alpha`, `blend_rect`, `blend_round_rect`, `fill_circle_aa`, `ring` — yangi AA grafika primitivlari.
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
make build       # debug build
make run         # QEMU virt da ishga tushirish (UART konsol)
make debug       # QEMU + gdb stub (port 1234)
make screenshot  # headless QEMU + framebuffer skrinshot (dist/screenshot.png)
make ios         # iPhone (UTM SE) uchun image tayyorlash
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

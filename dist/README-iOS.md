# ZaminOS iPhone'da (UTM SE orqali)

iPhone'da ZaminOS'ni **UTM SE: Retro PC emulator** (App Store, bepul) ilovasi orqali sinab ko'rish bo'yicha qo'llanma.

> **Eslatma**: UTM SE'da JIT yo'q (Apple ruxsat bermaydi), faqat **interpreter** rejimi ishlaydi. ZaminOS sekin ishlaydi, ammo bizning kichik kernel uchun bu masala emas — boot xabari muvaffaqiyatli chiqadi.

## 1. Faylni tayyorlash

Mac yoki Linux'da:

```bash
git clone <repo-url> ZaminOS
cd ZaminOS
make ios
```

Natijada `dist/` papkasida ikki fayl paydo bo'ladi:

- `zaminos-kernel.bin` — UTM SE uchun **xom binary** (tavsiya etiladi)
- `zaminos-kernel.elf` — debug uchun ELF (UTM toʻliq versiyasi uchun)

## 2. Faylni iPhone'ga ko'chirish

Bir nechta yo'l:

| Usul | Qadamlar |
|---|---|
| **iCloud Drive** | Mac'da `dist/zaminos-kernel.bin` ni iCloud Drive'ga sudrab tashlang. iPhone'da Files ilovasidan oching. |
| **AirDrop** | Mac'da faylga o'ng tugma → Share → AirDrop → iPhone'ingizga yuboring. |
| **Email / Telegram** | O'zingizga yuborib, iPhone'da yuklab oling. |

## 3. UTM SE'da yangi VM yaratish

1. **UTM SE** ilovasini oching.
2. Yuqori o'ngdagi **`+`** tugmasini bosing → **`Create New VM`**.
3. **`Custom`** ni tanlang.

### 3.1 Information bo'limi
- **Name**: `ZaminOS`
- **Architecture**: **`ARM64 (aarch64)`**
- **System**: **`virt`** (`QEMU 9.x ARM Virtual Machine`)

### 3.2 System bo'limi
- **CPU**: `cortex-a72` (yoki `Default`)
- **CPU Cores**: `1`
- **RAM**: `256 MB` (kerak emas, kichikroq ham bo'ladi)

### 3.3 Drives bo'limi
- **Hech qanday drive qo'shmang** — `Skip` yoki bo'sh qoldiring.
  ZaminOS'ga disk kerak emas.

### 3.4 Sharing bo'limi
- Skip.

### 3.5 Summary
- Nomini tasdiqlang va **`Save`** bosing.

## 4. Kernel ni VM ga ulash

VM yaratilgandan keyin uning **sozlamalariga** kiring (gear icon yoki uzun bosing → Edit):

1. **`System`** bo'limiga o'ting.
2. Pastga aylantiring — **`Boot`** yoki **`Linux Settings`** bo'limini toping.
3. **`Linux Kernel Image`** maydoniga `zaminos-kernel.bin` faylini tanlang (Files orqali).
4. **`Linux Initial Ramdisk`** — bo'sh qoldiring.
5. **`Linux Boot Arguments`** — bo'sh qoldiring.

## 5. Konsolni sozlash

ZaminOS hozircha grafika chiqarmaydi — faqat UART (serial) orqali yozadi.

1. Sozlamalardagi **`Display`** yoki **`Devices`** bo'limiga o'ting.
2. Display'ni **o'chiring** yoki **`Console (Serial)`** rejimini tanlang.
3. **`Serial`** qurilma yoqilgan bo'lsin (odatda standart yoqilgan).

## 6. Ishga tushirish

1. VM ro'yxatiga qayting.
2. ZaminOS ustiga bosing → **▶ Play** tugmasi.
3. UTM SE konsol oynasini ochadi va quyidagi xabarni ko'rasiz:

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

Yadro `wfe` (wait-for-event) tsiklida turadi — bu normal holat.

## Muammolarga yechim

| Muammo | Yechim |
|---|---|
| "Kernel not recognized" | `zaminos-kernel.bin` o'rniga `zaminos-kernel.elf` ni sinab ko'ring |
| Konsol bo'sh | Display'ni `Console (Serial)` ga o'zgartiring, Serial qurilma yoqilgan bo'lsin |
| Juda sekin | UTM SE'da JIT yo'q, bu normal. UTM toʻliq versiyasi (sideload) tezroq |
| RAM yetarli emas | RAM ni `512 MB` ga oshiring |

## Skrinshot kerakmi?

Sozlash jarayonida muammo bo'lsa, ekranni surat oling va ko'rsating — qadam-qadam tekshiramiz.

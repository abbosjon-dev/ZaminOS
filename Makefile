# ZaminOS Makefile
# Asosiy buyruqlar:
#   make build   — kernelni qurish (debug)
#   make release — release rejimi
#   make run     — QEMU virt mashinasida ishga tushirish
#   make debug   — QEMU ni gdb stub bilan ishga tushirish (-s -S)
#   make clean   — yig'ish artefaktlarini tozalash

TARGET       := aarch64-unknown-none-softfloat
PROFILE      ?= debug
CARGO_FLAGS  :=
ifeq ($(PROFILE),release)
    CARGO_FLAGS += --release
endif

KERNEL_ELF := target/$(TARGET)/$(PROFILE)/kernel
KERNEL_BIN := target/$(TARGET)/$(PROFILE)/kernel.bin

QEMU         := qemu-system-aarch64
QEMU_MACHINE := -M virt -cpu cortex-a72 -smp 1 -m 512M
QEMU_DEV     := -device ramfb \
                -device virtio-keyboard-device \
                -device virtio-tablet-device \
                -device virtio-mouse-device
QEMU_OUT     := -nographic -serial mon:stdio
QEMU_OPTS    := $(QEMU_MACHINE) $(QEMU_DEV) $(QEMU_OUT) -kernel $(KERNEL_ELF)

DIST_DIR := dist
IOS_ELF  := $(DIST_DIR)/zaminos-kernel.elf
IOS_BIN  := $(DIST_DIR)/zaminos-kernel.bin

.PHONY: all build release run debug clean objdump size bin ios screenshot

all: build

build:
	cargo build $(CARGO_FLAGS)

release:
	$(MAKE) build PROFILE=release

run: build
	$(QEMU) $(QEMU_OPTS)

debug: build
	$(QEMU) $(QEMU_OPTS) -s -S

bin: build
	rust-objcopy --strip-all -O binary $(KERNEL_ELF) $(KERNEL_BIN)

objdump: build
	rust-objdump -d $(KERNEL_ELF) | less

size: build
	rust-size $(KERNEL_ELF)

# Skrinshot olish: 6 ta desktop app + Esc bilan mobile rejim namoyishi.
screenshot: build
	@mkdir -p $(DIST_DIR)
	@rm -f /tmp/zaminos-mon.sock $(DIST_DIR)/shot-*.ppm $(DIST_DIR)/shot-*.png $(DIST_DIR)/screenshot.png
	@echo "==> QEMU ni headless rejimda ishga tushirish..."
	@$(QEMU) $(QEMU_MACHINE) $(QEMU_DEV) \
		-display none \
		-serial file:$(DIST_DIR)/serial.log \
		-monitor unix:/tmp/zaminos-mon.sock,server,nowait \
		-kernel $(KERNEL_ELF) & \
	QEMU_PID=$$!; \
	MON="socat - UNIX-CONNECT:/tmp/zaminos-mon.sock"; \
	sleep 2; \
	echo "==> [01] desktop / Welcome"; \
	echo "screendump $(DIST_DIR)/shot-01-desktop-welcome.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [02] desktop / Activity"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 1.0; \
	echo "screendump $(DIST_DIR)/shot-02-desktop-activity.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [03] desktop / Calculator"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 0.4; \
	for k in 1 2 j 3 4; do echo "sendkey $$k" | $$MON >/dev/null; sleep 0.1; done; \
	echo "screendump $(DIST_DIR)/shot-03-desktop-calculator.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [04] desktop / Keyboard"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 0.4; \
	for k in s a l o m; do echo "sendkey $$k" | $$MON >/dev/null; sleep 0.15; done; \
	echo "screendump $(DIST_DIR)/shot-04-desktop-keyboard.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [05] desktop / Terminal"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 0.4; \
	for k in h e l p; do echo "sendkey $$k" | $$MON >/dev/null; sleep 0.1; done; \
	echo "sendkey ret" | $$MON >/dev/null; sleep 0.3; \
	for k in p s; do echo "sendkey $$k" | $$MON >/dev/null; sleep 0.1; done; \
	echo "sendkey ret" | $$MON >/dev/null; sleep 0.3; \
	echo "screendump $(DIST_DIR)/shot-05-desktop-terminal.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [06] desktop / Paint"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 0.4; \
	for i in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15; do echo "mouse_move 8 0" | $$MON >/dev/null; sleep 0.03; done; \
	for i in 1 2 3 4 5 6 7 8 9 10 11 12; do echo "mouse_move 0 8" | $$MON >/dev/null; sleep 0.03; done; \
	for i in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15; do echo "mouse_move -8 0" | $$MON >/dev/null; sleep 0.03; done; \
	for i in 1 2 3 4 5 6 7 8 9 10 11 12; do echo "mouse_move 0 -8" | $$MON >/dev/null; sleep 0.03; done; \
	echo "screendump $(DIST_DIR)/shot-06-desktop-paint.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [07] desktop / Files"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 0.4; \
	for k in j j j; do echo "sendkey $$k" | $$MON >/dev/null; sleep 0.1; done; \
	echo "screendump $(DIST_DIR)/shot-07-desktop-files.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [08] desktop / Music"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 0.4; \
	echo "screendump $(DIST_DIR)/shot-08-desktop-music.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [09] desktop / Clock"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 3.0; \
	echo "screendump $(DIST_DIR)/shot-09-desktop-clock.ppm" | $$MON >/dev/null; sleep 0.5; \
	echo "==> [10] desktop / Settings"; \
	echo "sendkey tab" | $$MON >/dev/null; sleep 0.4; \
	for k in 4 minus minus equal equal equal; do echo "sendkey $$k" | $$MON >/dev/null; sleep 0.08; done; \
	echo "screendump $(DIST_DIR)/shot-10-desktop-settings.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [11] mobile / Home grid"; \
	echo "sendkey esc" | $$MON >/dev/null; sleep 0.5; \
	echo "screendump $(DIST_DIR)/shot-11-mobile-home.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [12] mobile / Welcome InApp"; \
	for i in 1 2 3 4 5 6 7 8 9; do echo "sendkey tab" | $$MON >/dev/null; sleep 0.08; done; \
	echo "sendkey ret" | $$MON >/dev/null; sleep 0.6; \
	echo "screendump $(DIST_DIR)/shot-12-mobile-welcome.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [13] mobile / Music InApp"; \
	echo "sendkey f1" | $$MON >/dev/null; sleep 0.3; \
	for i in 1 2 3 4 5 6 7; do echo "sendkey tab" | $$MON >/dev/null; sleep 0.08; done; \
	echo "sendkey ret" | $$MON >/dev/null; sleep 0.6; \
	echo "screendump $(DIST_DIR)/shot-13-mobile-music.ppm" | $$MON >/dev/null; sleep 0.3; \
	echo "==> [14] mobile / Clock InApp"; \
	echo "sendkey f1" | $$MON >/dev/null; sleep 0.3; \
	for i in 1 2; do echo "sendkey tab" | $$MON >/dev/null; sleep 0.1; done; \
	echo "sendkey ret" | $$MON >/dev/null; sleep 1.2; \
	echo "screendump $(DIST_DIR)/shot-14-mobile-clock.ppm" | $$MON >/dev/null; sleep 0.3; \
	kill $$QEMU_PID 2>/dev/null; \
	wait $$QEMU_PID 2>/dev/null; true
	@if command -v convert >/dev/null 2>&1; then \
		rm -f $(DIST_DIR)/shot-*.png; \
		for f in $(DIST_DIR)/shot-*.ppm; do convert $$f $${f%.ppm}.png; done; \
		cp $(DIST_DIR)/shot-01-desktop-welcome.png $(DIST_DIR)/screenshot.png 2>/dev/null || true; \
		ls -la $(DIST_DIR)/shot-*.png; \
	fi
	@echo "==> Serial log: $(DIST_DIR)/serial.log"

ios: release
	@mkdir -p $(DIST_DIR)
	cp target/$(TARGET)/release/kernel $(IOS_ELF)
	rust-objcopy --strip-all -O binary $(IOS_ELF) $(IOS_BIN)
	@echo
	@echo "==> iOS / UTM SE artifactlari tayyor:"
	@ls -lh $(IOS_ELF) $(IOS_BIN)
	@echo
	@echo "Sozlash bo'yicha qo'llanma:"
	@echo "  $(DIST_DIR)/README-iOS.md"

clean:
	cargo clean
	rm -rf $(DIST_DIR)/*.bin $(DIST_DIR)/*.elf

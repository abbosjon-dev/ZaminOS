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
                -device virtio-tablet-device
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

# Skrinshot olish: QEMU ni headless ishga tushirib, klaviatura/sichqoncha
# eventlarini monitor orqali yuborib, har bir app ekranini rasmga olish.
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
	echo "==> [1/3] Welcome ekran (boot oxiri)"; \
	echo "screendump $(DIST_DIR)/shot-1-welcome.ppm" | $$MON >/dev/null; \
	sleep 0.3; \
	echo "==> [2/3] TAB -> SysMon ekran"; \
	echo "sendkey tab" | $$MON >/dev/null; \
	sleep 1.2; \
	echo "screendump $(DIST_DIR)/shot-2-sysmon.ppm" | $$MON >/dev/null; \
	sleep 0.3; \
	echo "==> [3/3] TAB -> Keyboard, 'salom' yozish"; \
	echo "sendkey tab" | $$MON >/dev/null; \
	sleep 0.3; \
	for k in s a l o m; do \
		echo "sendkey $$k" | $$MON >/dev/null; \
		sleep 0.2; \
	done; \
	echo "mouse_move 25000 14000" | $$MON >/dev/null; \
	sleep 0.5; \
	echo "screendump $(DIST_DIR)/shot-3-keyboard.ppm" | $$MON >/dev/null; \
	sleep 0.3; \
	kill $$QEMU_PID 2>/dev/null; \
	wait $$QEMU_PID 2>/dev/null; true
	@if command -v convert >/dev/null 2>&1; then \
		for f in $(DIST_DIR)/shot-*.ppm; do convert $$f $${f%.ppm}.png; done; \
		cp $(DIST_DIR)/shot-1-welcome.png $(DIST_DIR)/screenshot.png; \
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

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
QEMU_DEV     := -device ramfb
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

# Skrinshot olish: QEMU ni -display none va monitor socket bilan ishga tushirib,
# bir necha sekunddan keyin `screendump` buyrug'ini yuborish, keyin PPM ni PNG ga
# o'tkazish. Natijada dist/screenshot.png hosil bo'ladi.
screenshot: build
	@mkdir -p $(DIST_DIR)
	@rm -f /tmp/zaminos-mon.sock $(DIST_DIR)/screenshot.ppm $(DIST_DIR)/screenshot.png
	@echo "==> QEMU ni headless rejimda ishga tushirish..."
	@$(QEMU) $(QEMU_MACHINE) $(QEMU_DEV) \
		-display none \
		-serial file:$(DIST_DIR)/serial.log \
		-monitor unix:/tmp/zaminos-mon.sock,server,nowait \
		-kernel $(KERNEL_ELF) & \
	QEMU_PID=$$!; \
	sleep 3; \
	echo "==> screendump yuborish..."; \
	echo "screendump $(DIST_DIR)/screenshot.ppm" | socat - UNIX-CONNECT:/tmp/zaminos-mon.sock; \
	sleep 1; \
	kill $$QEMU_PID 2>/dev/null; \
	wait $$QEMU_PID 2>/dev/null; true
	@if command -v convert >/dev/null 2>&1; then \
		convert $(DIST_DIR)/screenshot.ppm $(DIST_DIR)/screenshot.png && \
		echo "==> $(DIST_DIR)/screenshot.png ($$(stat -c %s $(DIST_DIR)/screenshot.png) bayt)"; \
	else \
		echo "==> $(DIST_DIR)/screenshot.ppm ($(DIST_DIR)/ ichida)"; \
		echo "    PNG uchun: sudo apt install imagemagick"; \
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

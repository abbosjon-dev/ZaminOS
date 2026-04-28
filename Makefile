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
QEMU_OUT     := -nographic -serial mon:stdio
QEMU_OPTS    := $(QEMU_MACHINE) $(QEMU_OUT) -kernel $(KERNEL_ELF)

DIST_DIR := dist
IOS_ELF  := $(DIST_DIR)/zaminos-kernel.elf
IOS_BIN  := $(DIST_DIR)/zaminos-kernel.bin

.PHONY: all build release run debug clean objdump size bin ios

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

TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := ../target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

elf:
ifeq ($(OS),Windows_NT)
	@python3 build.py
else
	@python3 build.py
endif

$(BINS): $(TARGET_DIR)/%.bin: $(TARGET_DIR)/%
	$(OBJCOPY) $< --strip-all -O binary $@

binary: elf $(BINS)

build: binary

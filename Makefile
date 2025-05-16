#* Makefile for running RISC-V and LoongArch QEMU with build steps

#! Can be override in cmd. E.g. make run-riscv OS_FILE=my_kernel.elf MEM=2G SMP=4 FS=rootfs.img DISK_IMG=data.img
OS_FILE ?= kernel-rv		# kernel file
MEM ?= 128M					# qemu mem size
SMP ?= 4					# qemu cpu core count
FS ?= fs.img				# file system image
DISK_IMG_RV ?= disk.img		# riscv's additional disk (optional)
DISK_IMG_LA ?= disk-la.img	# LoongArch's additional disk (optional)
KERNEL_BIN_PATH ?= target/riscv64gc-unknown-none-elf/release/Artemos
FS_IMG_PATH ?= target/riscv64gc-unknown-none-elf/release/fs.img
BOOTLOADER_PATH ?= ./bootloader/rustsbi-qemu.bin

# ----------------------------------------------------------------------
#! Riscv-64

build-riscv:
	@echo "Building riscv with: $(OS_FILE), mem: $(MEM), smp: $(SMP), fs: $(FS), disk: $(DISK_IMG_RV)"
	@cargo build --release --target riscv64gc-unknown-none-elf -Z build-std=core,alloc
	@rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/Artemos -O binary target/riscv64gc-unknown-none-elf/release/Artemos.bin
	@echo "Build finished."

run-riscv: build-riscv
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/Artemos.bin,addr=0x80200000 \
		-bios tools/opensbi.bin \
		-smp $(SMP)
	# @echo "Running RISC-V QEMU with kernel: $(OS_FILE), mem: $(MEM), smp: $(SMP), fs: $(FS), disk: $(DISK_IMG_RV)"
	# qemu-system-riscv64 \
	# 	-machine virt \
	# 	-kernel $(OS_FILE) \
	# 	-m $(MEM) \
	# 	-nographic \
	# 	-smp $(SMP) \
	# 	-bios default \
	# 	-drive file=$(FS),if=none,format=raw,id=x0 \
	# 	-device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
	# 	-no-reboot \
	# 	-device virtio-net-device,netdev=net \
	# 	-netdev user,id=net \
	# 	-rtc base=utc \
	# 	-drive file=$(DISK_IMG_RV),if=none,format=raw,id=x1 \
	# 	-device virtio-blk-device,drive=x1,bus=virtio-mmio-bus.1

dbg-riscv: build-riscv
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/Artemos.bin,addr=0x80200000 \
		-bios tools/opensbi.bin \
		-smp $(SMP) \
		-s -S
		# qemu-system-riscv64 \
		# -machine virt \
		# -kernel $(OS_FILE) \
		# -m $(MEM) \
		# -nographic \
		# -smp $(SMP) \
		# -bios default \
		# -drive file=$(FS),if=none,format=raw,id=x0 \
		# -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0 \
		# -no-reboot \
		# -device virtio-net-device,netdev=net \
		# -netdev user,id=net \
		# -rtc base=utc \
		# -drive file=$(DISK_IMG_RV),if=none,format=raw,id=x1 \
		# -device virtio-blk-device,drive=x1,bus=virtio-mmio-bus.1 \
		# -s -S


# ----------------------------------------------------------------------
#! Loongarch

build-loongarch:
	@echo "Building loongarch with $(OS_FILE), mem: $(MEM), smp: $(SMP), fs: $(FS), disk: $(DISK_IMG_RV)"
	# TODO
	@echo "Build finished."

run-loongarch:
	@echo "Running LoongArch QEMU with kernel: $(OS_FILE), mem: $(MEM), smp: $(SMP), fs: $(FS), disk: $(DISK_IMG_LA)"
	qemu-system-loongarch64 \
		-kernel $(OS_FILE) \
		-m $(MEM) \
		-nographic \
		-smp $(SMP) \
		-drive file=$(FS),if=none,format=raw,id=x0 \
		-device virtio-blk-pci,drive=x0,bus=virtio-mmio-bus.0 \
		-no-reboot \
		-device virtio-net-pci,netdev=net0 \
		-netdev user,id=net0,hostfwd=tcp::5555-:5555,hostfwd=udp::5555-:5555 \
		-rtc base=utc \
		-drive file=$(DISK_IMG_LA),if=none,format=raw,id=x1 \
		-device virtio-blk-pci,drive=x1,bus=virtio-mmio-bus.1

dbg-loongarch:
	@echo "Running LoongArch QEMU with kernel: $(OS_FILE), mem: $(MEM), smp: $(SMP), fs: $(FS), disk: $(DISK_IMG_LA)"
	qemu-system-loongarch64 \
		-kernel $(OS_FILE) \
		-m $(MEM) \
		-nographic \
		-smp $(SMP) \
		-drive file=$(FS),if=none,format=raw,id=x0 \
		-device virtio-blk-pci,drive=x0,bus=virtio-mmio-bus.0 \
		-no-reboot \
		-device virtio-net-pci,netdev=net0 \
		-netdev user,id=net0,hostfwd=tcp::5555-:5555,hostfwd=udp::5555-:5555 \
		-rtc base=utc \
		-drive file=$(DISK_IMG_LA),if=none,format=raw,id=x1 \
		-device virtio-blk-pci,drive=x1,bus=virtio-mmio-bus.1 \
		-s -S

clean:
	@echo "Cleaning up..."
	# rm -f your_compiled_files_here
	# TODO
	@cargo clean

.PHONY: build run dbg run-riscv run-loongarch clean

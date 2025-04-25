.PHONY: run

QEMU_ARGS := -machine virt \
			 -nographic \
			 -bios ./bootloader/rustsbi-qemu.bin \
			 -device loader,file=target/riscv64gc-unknown-none-elf/release/Artemos,addr=0x80200000 \
			 -drive file=target/riscv64gc-unknown-none-elf/release/fs.img,if=none,format=raw,id=x0 \
			 -device virtio-blk-device,drive=x0,bus=virtio-mmio-bus.0

build:
	@cd ./easy-fs-fuse && make
	@cd ./user && make
	@cargo build --release

run: build
	@qemu-system-riscv64 $(QEMU_ARGS)

dbg: build # run debug server. -s: shorthand for -gdb tcp::1234; -S freeze CPU at startup (use 'c' to start execution)
	@qemu-system-riscv64 $(QEMU_ARGS) -s -S
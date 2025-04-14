# Artemos 

## Setup 
1. configure the rust and qemu env ( temporarily, please refer to the according official guide ). You may refer to this tutorial: [https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html)
2. run the following command:
```bash
cd user
make build
cd ..
cargo build --release
qemu-system-riscv64 -machine virt \
    -nographic \
    -bios bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/Artemos
```
3. run qemu in debug mode (start tcp:1234)
```bash
cd user
make build
cd ..
cargo build --release
qemu-system-riscv64 -machine virt \
    -nographic \
    -bios bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/Artemos \
    -s \
    -S
```

If you want to exit the QEMU emulator, first press Ctrl+A, then press X
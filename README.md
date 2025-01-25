# Artemos 

## Setup 
1. configure the rust and qemu env ( temporarily, please refer to the according official guide ). You may refer to this tutorial: [https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html)
2. run the following command:
```bash
cd kernel && cargo build --release

cd ../

qemu-system-riscv64 \
     -machine virt \
     -nographic \
     -bios bootloader/rustsbi-qemu.bin \
     -device loader,file=target/riscv64gc-unknown-none-elf/release/kernel
```

If you want to exit the QEMU emulator, first press Ctrl+A, then press X
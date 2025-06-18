# 启动简单说明

## 编译

```
cargo build --release --target riscv64gc-unknown-none-elf -Z build-std=core,alloc
```

```
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/Artemos -O binary target/riscv64gc-unknown-none-elf/release/Artemos.bin
```

## 运行 qemu

```
qemu-system-riscv64 -machine virt -nographic -device loader,file=target/riscv64gc-unknown-none-elf/release/Artemos.bin,addr=0x80200000 -bios tools/opensbi.bin
```

如果要用 gsb 调试，还需要 -s -S 参数

-smp 参数指定核数

## 运行 gdb

```
riscv64-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/Artemos' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```

启动内核精简版一条指令（Mac / Linxu 下运行）

```
cargo build --release --target riscv64gc-unknown-none-elf -Z build-std=core,alloc && rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/Artemos -O binary target/riscv64gc-unknown-none-elf/release/Artemos.bin && qemu-system-riscv64 -machine virt -nographic -device loader,file=target/riscv64gc-unknown-none-elf/release/Artemos.bin,addr=0x80200000 -bios tools/opensbi.bin -smp 4
```

运行 gdb 调试

```
riscv64-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/Artemos' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
```

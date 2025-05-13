# 启动简单说明
## 编译
```
cargo build --release --target targets/riscv64.json -Z build-std=core,alloc
```

## 运行 qemu
```
qemu-system-riscv64 -machine virt -nographic -device loader,file=target/riscv64/release/Artemos
```
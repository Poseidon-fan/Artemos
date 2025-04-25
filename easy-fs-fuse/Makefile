.PHONY: run
DEFAULT_TARGET := $(shell rustc -vV | grep host | cut -d' ' -f2)

run:
ifeq ($(OS),Windows_NT)
	@set RUSTFLAGS= && cargo build --release
else
	@RUSTFLAGS="" CARGO_BUILD_TARGET="$(DEFAULT_TARGET)" cargo run --verbose --release -- -s ../user/src/bin/ -t ../target/riscv64gc-unknown-none-elf/release/
endif
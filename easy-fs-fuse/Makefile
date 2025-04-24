build:
	@if [ "$(OS)" = "Windows_NT" ]; then \
        if command -v powershell >/dev/null; then \
            powershell -Command "\$$env:RUSTFLAGS=\$$null; \$$env:CARGO_BUILD_TARGET=\$$null; cargo build --release"; \
        else \
            cmd /C "set RUSTFLAGS= && set CARGO_BUILD_TARGET= && cargo build --release"; \
        fi; \
    else \
        RUSTFLAGS= CARGO_BUILD_TARGET= cargo build --release; \
    fi
build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test --workspace -- --nocapture

lint:
	cargo clippy --workspace -- -D warnings

fmt:
	cargo fmt --check

.PHONY: build test lint fmt

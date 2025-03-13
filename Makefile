.PHONY: build

build:
	cargo build --release && ./target/release/my-interpreter

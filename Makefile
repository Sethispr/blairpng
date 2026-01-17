.PHONY: all build release run test clean fmt lint help

BINARY_NAME=blairpng

all: build

# debug mode
build:
	cargo build

release:
	cargo build --release

run:
	cargo run --release

run-opt:
	cargo run -- --directory "$(DIR)" --verbose

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

lint:
	cargo clippy

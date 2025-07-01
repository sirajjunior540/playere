.PHONY: help build run dev release clean install

help:
	@echo "YouTube Player - Build Commands"
	@echo "------------------------------"
	@echo "make build    - Build debug version"
	@echo "make run      - Run the application"
	@echo "make dev      - Run in development mode"
	@echo "make release  - Build release packages"
	@echo "make clean    - Clean build artifacts"
	@echo "make install  - Install dependencies"

build:
	cargo build

run:
	cargo run

dev:
	cargo tauri dev

release:
	@echo "Building release packages..."
	cargo tauri build

clean:
	cargo clean
	rm -rf target/

install:
	@echo "Installing dependencies..."
	cargo install tauri-cli
	cargo build
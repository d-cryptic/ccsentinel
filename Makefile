.PHONY: all build test lint fmt install clean dev-app changelog

all: build

build:
	cargo build

build-release:
	cargo build --release

test:
	cargo nextest run

test-watch:
	cargo watch -x 'nextest run'

lint:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt --all

audit:
	cargo audit

install:
	cargo install --path crates/cst-cli

clean:
	cargo clean

dev-app:
	cd apps/desktop && bun run tauri dev

build-app:
	cd apps/desktop && bun run tauri build

changelog:
	git-cliff -o CHANGELOG.md

# Run all checks (pre-commit)
check: fmt lint test
	@echo "✓ All checks passed"

.PHONY: build release test lint fmt check clean install bench

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

check: fmt-check lint test

clean:
	cargo clean

install:
	cargo install --path .

bench:
	cargo bench

deny:
	cargo deny check

changelog:
	git cliff -o CHANGELOG.md

size:
	cargo build --release && ls -lh target/release/binrs

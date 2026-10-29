.PHONY: all test build check clippy clean run publish

all: clean check test build run

test:
	RUST_BACKTRACE=1 cargo test --release && exit

build:
	cargo build --release -j 8 && exit

check:
	cargo check --release --tests && exit

clippy:
	cargo clippy --release --all-targets -- -D warnings && exit

clean:
	cargo clean && exit

run:
	cargo run --release --package telegram-api-rs && exit

publish:
	cargo publish && exit

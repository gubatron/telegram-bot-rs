.PHONY: all test build check clippy clean run publish

# cargo-sweep (cargo install cargo-sweep), when installed, deletes whatever an older build
# left in target/ that this build didn't use, so only fresh release artifacts remain.
SWEEP := $(shell command -v cargo-sweep >/dev/null 2>&1 && echo cargo sweep)
all: clean check test build run

test:
	RUST_BACKTRACE=1 cargo test --release && exit

build:
	$(if $(SWEEP),$(SWEEP) --stamp)
	cargo build --release -j 8
	$(if $(SWEEP),$(SWEEP) --file)

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

FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$$ | awk -F '"' '{print $$2}')

all:
	cargo build --target $(FORMAT)


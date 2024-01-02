SHELL=/bin/bash
PRODUCT=$(shell basename $$(pwd))
FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$ | awk -F '"' '{print $$2}')
TARGET=target/$(FORMAT)/release/$(PRODUCT).efi

# Build a bootloader.
$(TARGET): $(shell find . -type f | grep -vf <(git ls-files --other))
	cargo build --release --target $(FORMAT)

# Get a bootloader file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


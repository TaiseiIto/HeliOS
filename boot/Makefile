PRODUCT=$(shell basename $$(pwd))
FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$ | awk -F '"' '{print $$2}')
TARGET=target/$(FORMAT)/release/$(PRODUCT).efi

# Build a bootloader.
$(TARGET): $(git ls-files)
	cargo build --release --target $(FORMAT)

# Get a bootloader file path.
.PHONY: target
target:
	@echo $(realpath $(TARGET))


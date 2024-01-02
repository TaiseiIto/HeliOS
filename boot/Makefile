SHELL=/bin/bash
PRODUCT=$(shell basename $$(pwd))
FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$ | awk -F '"' '{print $$2}')
TARGET=target/$(FORMAT)/release/$(PRODUCT).efi
DOCUMENT=target/doc/boot/index.html

# Build a bootloader.
$(TARGET): $(shell find . -type f | grep -vf <(git ls-files --other))
	cargo build --release --target $(FORMAT)

# Generate documents.
# Usage: $ make doc
.PHONY: doc
doc: $(DOCUMENT)

# Generate documents.
$(DOCUMENT): $(shell find . -type f | grep -vf <(git ls-files --other))
	cargo doc

# Get a bootloader file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


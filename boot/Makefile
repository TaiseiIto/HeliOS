include ../.make/common.mk

FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$$ | awk -F '"' '{print $$2}')
TARGET=target/$(FORMAT)/debug/$(PRODUCT).efi
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build a bootloader.
$(TARGET): $(call source_files, .)
	KERNEL=$(KERNEL) cargo build --target $(FORMAT)

# Clippy rust codes.
.PHONY: clippy
clippy:
	KERNEL=$(KERNEL) cargo clippy

# Generate documents.
# Usage: $ make doc
.PHONY: doc
doc: $(DOCUMENT)

# Generate documents.
$(DOCUMENT): $(call source_files, .)
	KERNEL=$(KERNEL) cargo doc

# Get a bootloader file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


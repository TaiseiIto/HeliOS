include ../.make/common.mk

FORMAT=$(shell sed -n "s/^\s*targets\s*=\s*\[\"\(.*\)\"\]$$/\1/p" rust-toolchain.toml)
TARGET=target/$(FORMAT)/debug/$(PRODUCT).efi
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build a bootloader.
$(TARGET): $(call SOURCE_FILES, .)
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
$(DOCUMENT): $(call SOURCE_FILES, .)
	KERNEL=$(KERNEL) cargo doc

# Get a bootloader file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


include ../../.make/common.mk

FORMAT=$(shell sed -n "s/^\s*targets\s*=\s*\[\"\(.*\)\"\]$$/\1/p" rust-toolchain.toml)
TARGET=target/$(FORMAT)/debug/$(PRODUCT)
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build the application.
$(TARGET): $(call SOURCE_FILES, .)
	cargo build --target $(FORMAT)

# Clippy rust codes.
.PHONY: clippy
clippy:
	cargo clippy

# Generate documents.
# Usage: $make doc
.PHONY: doc
doc: $(DOCUMENT)

# Get the application file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


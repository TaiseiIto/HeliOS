include ../../.make/common.mk

FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$$ | awk -F '"' '{print $$2}')
TARGET=target/$(FORMAT)/debug/$(PRODUCT)
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build the application.
$(TARGET): $(call source_files, .)
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


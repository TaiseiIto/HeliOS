include ../.make/common.mk

TARGET=target/$(shell ls *.json | awk -F '.' '{print $$1}')/debug/$(PRODUCT).elf
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build a kernel.
$(TARGET): $(call source_files, .)
	rustup component add rust-src --toolchain nightly-2024-01-01-x86_64-unknown-linux-gnu
	cargo build

# Clippy rust codes.
.PHONY: clippy
clippy:
	cargo clippy

# Generate documents.
# Usage: $make doc
.PHONY: doc
doc: $(DOCUMENT)

# Generate documents.
$(DOCUMENT): $(call source_files, .)
	cargo doc

# Get a kernel file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


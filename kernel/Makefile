include $(shell git rev-parse --show-toplevel)/.make/header.mk

TARGET=target/$(basename $(wildcard *.json))/debug/$(PRODUCT).elf
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build a kernel.
$(TARGET): $(call SOURCE_FILES, .)
	rustup component add rust-src --toolchain nightly-2024-01-01-x86_64-unknown-linux-gnu
	cargo build

# Clippy rust codes.
.PHONY: clippy
clippy:
	cargo clippy

# Format rust codes.
.PHONY: fmt
fmt:
	cargo fmt

# Generate documents.
# Usage: $make doc
.PHONY: doc
doc: $(DOCUMENT)

# Generate documents.
$(DOCUMENT): $(call SOURCE_FILES, .)
	cargo doc

include $(shell git rev-parse --show-toplevel)/.make/footer.mk


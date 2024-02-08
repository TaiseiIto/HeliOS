SHELL=/bin/bash
PRODUCT=$(shell basename $$(pwd))
TARGET=target/$(shell ls | grep ^.*\.json$$ | awk -F '.' '{print $$1}')/debug/$(PRODUCT).elf
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build a kernel.
$(TARGET): $(shell find . -type f | grep -vf <(git ls-files --exclude-standard --ignored -o))
	rustup component add rust-src --toolchain nightly-2024-01-01-x86_64-unknown-linux-gnu
	cargo build

# Generate documents.
# Usage: $make doc
.PHONY: doc
doc: $(DOCUMENT)

# Generate documents.
$(DOCUMENT): $(shell find . -type f | grep -vf <(git ls-files --exclude-standard --ignored -o))
	cargo doc

# Get a kernel file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


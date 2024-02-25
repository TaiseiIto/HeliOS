SHELL=/bin/bash
PRODUCT=$(shell basename $$(pwd))
FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$$ | awk -F '"' '{print $$2}')
TARGET=target/$(FORMAT)/debug/$(PRODUCT)
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build the application.
$(TARGET): $(shell find . -type f | grep -vf <(git ls-files --exclude-standard --ignored -o))
	cargo build --target $(FORMAT)

# Generate documents.
# Usage: $make doc
.PHONY: doc
doc: $(DOCUMENT)

# Get the application file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


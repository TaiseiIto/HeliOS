SHELL=/bin/bash
PRODUCT=$(shell basename $$(pwd))
FORMAT=$(shell cat rust-toolchain.toml | grep ^targets.*$$ | awk -F '"' '{print $$2}')
TARGET=target/$(FORMAT)/debug/$(PRODUCT).efi
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build a bootloader.
$(TARGET): $(shell find . -type f | grep -vf <(git ls-files --exclude-standard --ignored -o))
	APPLICATION_PROCESSOR_BOOT_LOADER_BASE=$(APPLICATION_PROCESSOR_BOOT_LOADER_BASE) APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR=$(APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR) KERNEL=$(KERNEL) cargo build --target $(FORMAT)

# Generate documents.
# Usage: $ make doc
.PHONY: doc
doc: $(DOCUMENT)

# Generate documents.
$(DOCUMENT): $(shell find . -type f | grep -vf <(git ls-files --exclude-standard --ignored -o))
	cargo doc

# Get a bootloader file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


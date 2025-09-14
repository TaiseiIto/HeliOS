include $(shell git rev-parse --show-toplevel)/.make/header.mk

FORMAT=$(shell sed -n "s/^\s*targets\s*=\s*\[\"\(.*\)\"\]$$/\1/p" rust-toolchain.toml)
TARGET=target/$(FORMAT)/debug/$(PRODUCT).efi
DOCUMENT=target/doc/$(PRODUCT)/index.html

# Build a bootloader.
$(TARGET): $(call SOURCE_FILES, .)
	PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) PROCESSOR_KERNEL=$(PROCESSOR_KERNEL) KERNEL=$(KERNEL) cargo build --target $(FORMAT)

# Clippy rust codes.
.PHONY: clippy
clippy:
	PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) PROCESSOR_KERNEL=$(PROCESSOR_KERNEL) KERNEL=$(KERNEL) cargo clippy

# Format rust codes.
.PHONY: fmt
fmt:
	PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) PROCESSOR_KERNEL=$(PROCESSOR_KERNEL) KERNEL=$(KERNEL) cargo fmt

# Generate documents.
# Usage: $ make doc
.PHONY: doc
doc: $(DOCUMENT)

# Generate documents.
$(DOCUMENT): $(call SOURCE_FILES, .)
	PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) PROCESSOR_KERNEL=$(PROCESSOR_KERNEL) KERNEL=$(KERNEL) cargo doc

include $(shell git rev-parse --show-toplevel)/.make/footer.mk


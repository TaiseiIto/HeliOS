PRODUCT=$(shell basename $$(pwd))
TARGET=$(shell cat rust-toolchain.toml | grep ^targets.*$ | awk -F '"' '{print $$2}')
OUTPUT=target/$(TARGET)/release/$(PRODUCT).efi

OUTPUT:
	cargo build --target $(TARGET)


# VNC port to interact with QEMU running on development environment.
VNC_PORT=5900

# Product name
PRODUCT=$(shell basename $$(pwd) | awk '{print tolower($$0)}')

# An OS image file name
TARGET=$(PRODUCT:=.img)

# block size in the OS image
BLOCK_SIZE=4K

# A number of blocks in the OS image
BLOCK_COUNT=4K

# A mount directory to build the OS image
MOUNT_DIRECTORY=$(PRODUCT)

# Build an OS image.
$(TARGET): $(shell git ls-files)
	dd if=/dev/zero of=$@ ibs=$(BLOCK_SIZE) count=$(BLOCK_COUNT)
	mkfs.fat $@
	mkdir $(MOUNT_DIRECTORY)
	mount -o loop $@ $(MOUNT_DIRECTORY)
	mkdir -p $(MOUNT_DIRECTORY)/EFI/BOOT
	umount $(MOUNT_DIRECTORY)
	rm -rf $(MOUNT_DIRECTORY)

# Build and enter development environment as a Docker container.
# Usage: $ make environment
.PHONY: build_environment
environment:
	make build -C .docker VNC_PORT=$(VNC_PORT)

# Delete development environment.
# Usage: $ make delete_environment
.PHONY: delete_environment
delete_environment:
	make delete -C .docker

# Rebuild and enter development environment.
# Usage: $ make rebuild_environment
.PHONY: rebuild_environment
rebuild_environment:
	make rebuild -C .docker

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
.PHONY: permission
permission:
	make permission -C .docker SSHKEY=$(realpath $(SSHKEY)) GPGKEY=$(realpath $(GPGKEY))


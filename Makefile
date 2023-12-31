# Product name
PRODUCT=$(shell basename $$(pwd) | awk '{print tolower($$0)}')

# An OS image file name
TARGET=$(PRODUCT:=.img)

# block size in the OS image
BLOCK_SIZE=4K

# A number of blocks in the OS image
BLOCK_COUNT=1K

# A mount directory to build the OS image
MOUNT_DIRECTORY=$(PRODUCT)

# A bootloader file path
BOOTLOADER_SOURCE=$(shell make target -C boot -s)
BOOTLOADER_DESTINATION=$(MOUNT_DIRECTORY)/EFI/BOOT/BOOTX64.EFI

# VNC port to interact with QEMU running on development environment.
VNC_PORT=5900

# Debug port to debug the OS on QEMU by GDB.
DEBUG_PORT=2159

# Build an OS image.
# Usage: $ make
$(TARGET): $(shell git ls-files)
	rm -f $@
	dd if=/dev/zero of=$@ ibs=$(BLOCK_SIZE) count=$(BLOCK_COUNT)
	mkfs.fat $@
	mkdir $(MOUNT_DIRECTORY)
	mount -o loop $@ $(MOUNT_DIRECTORY)
	mkdir -p $(shell dirname $(BOOTLOADER_DESTINATION))
	cp $(BOOTLOADER_SOURCE) $(BOOTLOADER_DESTINATION)
	umount $(MOUNT_DIRECTORY)
	rm -rf $(MOUNT_DIRECTORY)

# Run the OS on QEMU.
# Usage: make run
.PHONY: run
run:
	make run -C .tmux

# Stop the OS on QEMU.
# Usage: make stop
.PHONY: stop
stop:
	make stop -C .tmux

# Debug the OS on QEMU by GDB.
# Usage: make debug
.PHONY: debug
debug:
	make debug -C .tmux

# Build and enter development environment as a Docker container.
# Usage: $ make environment
.PHONY: build_environment
environment:
	make build -C .docker VNC_PORT=$(VNC_PORT) DEBUG_PORT=$(DEBUG_PORT)

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

# Get an OS image file name
# Usage: $ make target
.PHONY: target
target:
	@echo $(TARGET)


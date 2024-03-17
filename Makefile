SHELL=/bin/bash

# Product name
PRODUCT=$(shell basename $$(pwd))

# An OS image file name
TARGET=$(PRODUCT).img

# block size in the OS image
BLOCK_SIZE=4K

# A number of blocks in the OS image
BLOCK_COUNT=8K

# A mount directory to build the OS image
MOUNT_DIRECTORY=$(PRODUCT).mnt

# Operating system directory
OS_DIRECTORY=$(MOUNT_DIRECTORY)/$(PRODUCT)

# Application processor boot loader
APPLICATION_PROCESSOR_BOOT_LOADER_DIRECTORY=application_processor/boot
APPLICATION_PROCESSOR_BOOT_LOADER_SOURCE=$(shell make target -C $(APPLICATION_PROCESSOR_BOOT_LOADER_DIRECTORY) -s)
APPLICATION_PROCESSOR_BOOT_LOADER_DESTINATION=$(OS_DIRECTORY)/application_processor/boot/loader.bin
APPLICATION_PROCESSOR_BOOT_LOADER_BASE=$(shell make base -C $(APPLICATION_PROCESSOR_BOOT_LOADER_DIRECTORY) -s)
APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR=$(shell make stack_floor -C $(APPLICATION_PROCESSOR_BOOT_LOADER_DIRECTORY) -s)

# Applications
APPLICATION_SOURCE_DIRECTORY=applications
APPLICATION_DESTINATION_DIRECTORY=$(MOUNT_DIRECTORY)/applications
APPLICATIONS=$(shell ls $(APPLICATION_SOURCE_DIRECTORY))
APPLICATION_DESTINATIONS=$(foreach APPLICATION,$(APPLICATIONS),$(call application2destination,$(APPLICATION)))

define application2destination
	$(APPLICATION_DESTINATION_DIRECTORY)/$(1).elf
endef

define destination2source
	$(shell make target -C $(APPLICATION_SOURCE_DIRECTORY)/$(basename $(notdir $(1))) -s)
endef

# A bootloader file path
BOOT_DIRECTORY=boot
BOOTLOADER_SOURCE=$(shell make target -C $(BOOT_DIRECTORY) -s)
BOOTLOADER_DESTINATION=$(MOUNT_DIRECTORY)/EFI/BOOT/BOOTX64.EFI

# A kernel file path
KERNEL_DIRECTORY=kernel
KERNEL_SOURCE=$(shell make target -C $(KERNEL_DIRECTORY) -s)
KERNEL_DESTINATION=$(OS_DIRECTORY)/kernel.elf

# VNC port to interact with QEMU running on development environment.
VNC_PORT=5900

# Debug port to debug the OS on QEMU by GDB.
DEBUG_PORT=2159

# Telnet port to stop QEMU.
TELNET_PORT=23

# Build an OS image runs on QEMU.
# Usage: $ make
$(TARGET): $(shell find . -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	rm -f $@
	if mountpoint -q $(MOUNT_DIRECTORY); then umount -l $(MOUNT_DIRECTORY); fi
	rm -rf $(MOUNT_DIRECTORY)
	dd if=/dev/zero of=$@ ibs=$(BLOCK_SIZE) count=$(BLOCK_COUNT)
	mkfs.fat $@
	mkdir $(MOUNT_DIRECTORY)
	$(SUDO) mount -o loop $@ $(MOUNT_DIRECTORY)
	make $(APPLICATION_PROCESSOR_BOOT_LOADER_DESTINATION) SUDO=$(SUDO)
	make $(BOOTLOADER_DESTINATION) APPLICATION_PROCESSOR_BOOT_LOADER_BASE=$(APPLICATION_PROCESSOR_BOOT_LOADER_BASE) APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR=$(APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR) SUDO=$(SUDO)
	make $(KERNEL_DESTINATION) SUDO=$(SUDO)
	for application in $(APPLICATIONS); do make -C $(APPLICATION_SOURCE_DIRECTORY)/$$application; done
	$(SUDO) mkdir -p $(APPLICATION_DESTINATION_DIRECTORY)
	make $(APPLICATION_DESTINATIONS) SUDO=$(SUDO)
	$(SUDO) umount $(MOUNT_DIRECTORY)
	rm -rf $(MOUNT_DIRECTORY)

$(MOUNT_DIRECTORY): $(shell find . -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	if mountpoint -q $@; then umount -l $@; fi
	rm -rf $@
	mkdir $@
	make $(APPLICATION_PROCESSOR_BOOT_LOADER_DESTINATION)
	make $(BOOTLOADER_DESTINATION) APPLICATION_PROCESSOR_BOOT_LOADER_BASE=$(APPLICATION_PROCESSOR_BOOT_LOADER_BASE) APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR=$(APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR) 
	make $(KERNEL_DESTINATION)
	for application in $(APPLICATIONS); do make -C $(APPLICATION_SOURCE_DIRECTORY)/$$application; done
	mkdir -p $(APPLICATION_DESTINATION_DIRECTORY)
	make $(APPLICATION_DESTINATIONS)

$(APPLICATION_DESTINATION_DIRECTORY)/%.elf:
	$(SUDO) cp $(call destination2source,$@) $@

$(APPLICATION_PROCESSOR_BOOT_LOADER_DESTINATION): $(APPLICATION_PROCESSOR_BOOT_LOADER_SOURCE)
	$(SUDO) mkdir -p $(shell dirname $@)
	$(SUDO) cp $^ $@

$(APPLICATION_PROCESSOR_BOOT_LOADER_SOURCE): $(shell find $(APPLICATION_PROCESSOR_BOOT_LOADER_DIRECTORY) -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	make -C $(APPLICATION_PROCESSOR_BOOT_LOADER_DIRECTORY) APPLICATION_PROCESSOR_BOOT_LOADER_BASE=$(APPLICATION_PROCESSOR_BOOT_LOADER_BASE) APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR=$(APPLICATION_PROCESSOR_BOOT_LOADER_STACK_FLOOR) 

$(BOOTLOADER_DESTINATION): $(BOOTLOADER_SOURCE)
	$(SUDO) mkdir -p $(shell dirname $@)
	$(SUDO) cp $^ $@

$(BOOTLOADER_SOURCE): $(shell find $(BOOT_DIRECTORY) -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	make -C $(BOOT_DIRECTORY)

$(KERNEL_DESTINATION): $(KERNEL_SOURCE)
	$(SUDO) mkdir -p $(shell dirname $@)
	$(SUDO) cp $^ $@

$(KERNEL_SOURCE): $(shell find $(KERNEL_DIRECTORY) -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	make -C $(KERNEL_DIRECTORY)

# Build and enter development environment as a Docker container.
# Usage: $ make environment
.PHONY: build_environment
environment:
	make build -C .docker VNC_PORT=$(VNC_PORT) DEBUG_PORT=$(DEBUG_PORT)

# Debug the OS on QEMU by GDB.
# Usage: make debug
.PHONY: debug
debug: $(TARGET)
	-make debug -C .tmux -s

# Run the OS on QEMU.
# This target is called from .tmux/run.conf
# Don't execute this directly.
.PHONY: debug_on_tmux
debug_on_tmux:
	-make debug -C .qemu OS_PATH=$(realpath $(TARGET)) OS_NAME=$(PRODUCT) DEBUG_PORT=$(DEBUG_PORT) TELNET_PORT=$(TELNET_PORT) -s

# Debug QEMU by GDB.
# Usage: make debug_qemu
.PHONY: debug_qemu
debug_qemu: $(TARGET)
	-make debug_qemu -C .tmux -s

# Run the OS on QEMU.
# This target is called from .tmux/run.conf
# Don't execute this directly.
.PHONY: debug_qemu_on_tmux
debug_qemu_on_tmux:
	-make debug_qemu -C .qemu OS_PATH=$(realpath $(TARGET)) OS_NAME=$(PRODUCT) TELNET_PORT=$(TELNET_PORT) -s

# Delete all "#[allow(dead_code)]" lines
.PHONY: delete_allow_dead_code
delete_allow_dead_code:
	for i in $$(git ls-files | grep ^.*\.rs$$); do sed -i '/#\[allow(dead_code)\]/d' $$i; done

# Delete development environment.
# Usage: $ make delete_environment
.PHONY: delete_environment
delete_environment:
	make delete -C .docker

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
.PHONY: permission
permission:
	make permission -C .docker SSHKEY=$(realpath $(SSHKEY)) GPGKEY=$(realpath $(GPGKEY))

# Rebuild and enter development environment.
# Usage: $ make rebuild_environment
.PHONY: rebuild_environment
rebuild_environment:
	make rebuild -C .docker VNC_PORT=$(VNC_PORT) DEBUG_PORT=$(DEBUG_PORT)

# Run the OS on QEMU.
# Usage: make run
.PHONY: run
run: $(TARGET)
	-make run -C .tmux -s

# Run the OS on QEMU.
# This target is called from .tmux/run.conf
# Don't execute this directly.
.PHONY: run_on_tmux
run_on_tmux:
	-make run -C .qemu OS_PATH=$(realpath $(TARGET)) OS_NAME=$(PRODUCT) TELNET_PORT=$(TELNET_PORT) -s

# Stop the OS on QEMU.
# Usage: make stop
.PHONY: stop
stop:
	-make stop -C .tmux

# Stop the OS on QEMU.
# This target is called from .tmux/Makefile
# Don't execute this directly.
.PHONY: stop_on_tmux
stop_on_tmux:
	-make stop -C .qemu TELNET_PORT=$(TELNET_PORT)

# Get an OS image file name.
# Usage: $ make target
.PHONY: target
target:
	@echo $(TARGET)

# Build an OS directory to run on VirtualBox or VMware.
# Usage: $ make tree
.PHONY: tree
tree: $(MOUNT_DIRECTORY)


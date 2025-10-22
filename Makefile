include $(shell git rev-parse --show-toplevel)/.make/header.mk

# block size in the OS image
BLOCK_SIZE=4K

# A number of blocks in the OS image
BLOCK_COUNT=16K

MEDIA_SIZE=$(shell numfmt --to=iec $$(($$(numfmt --from=iec $(BLOCK_COUNT)) * $$(numfmt --from=iec $(BLOCK_SIZE)))))

# Target directory where the OS will be built.
TARGET=$(PRODUCT)

# Operating system directory
OS_DIRECTORY=$(TARGET)/$(PRODUCT)

# Application processor boot loader
PROCESSOR_DIRECTORY=processor
PROCESSOR_BOOT_LOADER_DIRECTORY=$(PROCESSOR_DIRECTORY)/boot
PROCESSOR_BOOT_LOADER_SOURCE=$(call SUB_TARGET, $(PROCESSOR_BOOT_LOADER_DIRECTORY))
PROCESSOR_BOOT_LOADER_DESTINATION=$(OS_DIRECTORY)/$(PROCESSOR_DIRECTORY)/boot_loader.bin
PROCESSOR_BOOT_LOADER=$(shell echo $(PROCESSOR_BOOT_LOADER_DESTINATION) | cut -d '/' -f 2-)

# Application processor kernel
PROCESSOR_KERNEL_DIRECTORY=$(PROCESSOR_DIRECTORY)/kernel
PROCESSOR_KERNEL_SOURCE=$(call SUB_TARGET, $(PROCESSOR_KERNEL_DIRECTORY))
PROCESSOR_KERNEL_DESTINATION=$(OS_DIRECTORY)/$(PROCESSOR_DIRECTORY)/kernel.elf
PROCESSOR_KERNEL=$(shell echo $(PROCESSOR_KERNEL_DESTINATION) | cut -d '/' -f 2-)

# Applications
APPLICATION_SOURCE_DIRECTORY=applications
APPLICATION_DESTINATION_DIRECTORY=$(TARGET)/applications
APPLICATIONS=$(wildcard $(APPLICATION_SOURCE_DIRECTORY)/*)
APPLICATION_DESTINATIONS=$(addprefix $(APPLICATION_DESTINATION_DIRECTORY)/, $(addsuffix .elf, $(notdir $(APPLICATIONS))))

# A bootloader file path
BOOTLOADER=EFI/BOOT/BOOTX64.EFI
BOOTLOADER_DIRECTORY=boot
BOOTLOADER_SOURCE=$(call SUB_TARGET, $(BOOTLOADER_DIRECTORY))
BOOTLOADER_DESTINATION=$(TARGET)/$(BOOTLOADER)

# A kernel file path
KERNEL_DIRECTORY=kernel
KERNEL_SOURCE=$(call SUB_TARGET, $(KERNEL_DIRECTORY))
KERNEL_DESTINATION=$(OS_DIRECTORY)/kernel.elf
KERNEL=$(shell echo $(KERNEL_DESTINATION) | cut -d '/' -f 2-)

# VNC port to interact with QEMU running on development environment.
VNC_PORT=5900

# Debug port to debug the OS on QEMU by GDB.
DEBUG_PORT=2159

# Telnet port to stop QEMU.
TELNET_PORT=23

$(TARGET): $(call SOURCE_FILES, .)
	rm -rf $@
	mkdir $@
	make $(PROCESSOR_BOOT_LOADER_DESTINATION)
	make $(PROCESSOR_KERNEL_DESTINATION)
	make $(BOOTLOADER_DESTINATION) PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) PROCESSOR_KERNEL=$(PROCESSOR_KERNEL) KERNEL=$(KERNEL)
	make $(KERNEL_DESTINATION)
	for application in $(APPLICATIONS); do make -C $$application; done
	mkdir -p $(APPLICATION_DESTINATION_DIRECTORY)
	make $(APPLICATION_DESTINATIONS)

$(APPLICATION_DESTINATION_DIRECTORY)/%.elf:
	cp $(call SUB_TARGET, $(APPLICATION_SOURCE_DIRECTORY)/$(basename $(notdir $@))) $@

$(PROCESSOR_BOOT_LOADER_DESTINATION): $(PROCESSOR_BOOT_LOADER_SOURCE)
	mkdir -p $(dir $@)
	cp $^ $@

$(PROCESSOR_BOOT_LOADER_SOURCE): $(call SOURCE_FILES, $(PROCESSOR_BOOT_LOADER_DIRECTORY))
	make -C $(PROCESSOR_BOOT_LOADER_DIRECTORY)

$(PROCESSOR_KERNEL_DESTINATION): $(PROCESSOR_KERNEL_SOURCE)
	mkdir -p $(dir $@)
	cp $^ $@

$(PROCESSOR_KERNEL_SOURCE): $(call SOURCE_FILES, $(PROCESSOR_KERNEL_DIRECTORY))
	make -C $(PROCESSOR_KERNEL_DIRECTORY)

$(BOOTLOADER_DESTINATION): $(BOOTLOADER_SOURCE)
	mkdir -p $(dir $@)
	cp $^ $@

$(BOOTLOADER_SOURCE): $(call SOURCE_FILES, $(BOOTLOADER_DIRECTORY))
	make -C $(BOOTLOADER_DIRECTORY) PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) KERNEL=$(KERNEL)

$(KERNEL_DESTINATION): $(KERNEL_SOURCE)
	mkdir -p $(dir $@)
	cp $^ $@

$(KERNEL_SOURCE): $(call SOURCE_FILES, $(KERNEL_DIRECTORY))
	make -C $(KERNEL_DIRECTORY)

# Build the OS on GitHub Actions.
# This target is called from .github/workflow/build.yml
# Don't execute this directly.
.PHONY: build_on_github
build_on_github:
	make build_on_github -C .docker VNC_PORT=$(VNC_PORT) DEBUG_PORT=$(DEBUG_PORT)

# Build and enter development environment as a Docker container.
# Usage: $ make environment
.PHONY: environment
environment:
	make build -C .docker VNC_PORT=$(VNC_PORT) DEBUG_PORT=$(DEBUG_PORT)

# Clippy rust codes.
.PHONY: clippy
clippy:
	make clippy -C $(BOOTLOADER_DIRECTORY) PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) PROCESSOR_KERNEL=$(PROCESSOR_KERNEL) KERNEL=$(KERNEL)
	make clippy -C $(KERNEL_DIRECTORY)
	make clippy -C $(PROCESSOR_KERNEL_DIRECTORY)
	for application in $(APPLICATIONS); do make clippy -C $$application; done

# Format rust codes.
.PHONY: fmt
fmt:
	make fmt -C $(BOOTLOADER_DIRECTORY) PROCESSOR_BOOT_LOADER=$(PROCESSOR_BOOT_LOADER) PROCESSOR_KERNEL=$(PROCESSOR_KERNEL) KERNEL=$(KERNEL)
	make fmt -C $(KERNEL_DIRECTORY)
	make fmt -C $(PROCESSOR_KERNEL_DIRECTORY)
	for application in $(APPLICATIONS); do make fmt -C $$application; done

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
	-make debug -C .qemu OS_PATH=$(abspath $(TARGET)) OS_NAME=$(PRODUCT) DEBUG_PORT=$(DEBUG_PORT) TELNET_PORT=$(TELNET_PORT) -s

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
	-make debug_qemu -C .qemu OS_PATH=$(abspath $(TARGET)) OS_NAME=$(PRODUCT) TELNET_PORT=$(TELNET_PORT) -s

# Delete all "#[allow(dead_code)]" lines
.PHONY: delete_allow_dead_code
delete_allow_dead_code:
	sed -i '/#\[allow(dead_code)\]/d' $$(git ls-files *.rs)

# Delete development environment.
# Usage: $ make delete_environment
.PHONY: delete_environment
delete_environment:
	make delete -C .docker

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage
# $ git config user.email someone@some.domain
# $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
.PHONY: permission
permission:
	make permission -C .docker SSHKEY=$(abspath $(SSHKEY)) GPGKEY=$(abspath $(GPGKEY))

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
	-make run -C .qemu OS_PATH=$(abspath $(TARGET)) OS_NAME=$(PRODUCT) TELNET_PORT=$(TELNET_PORT) -s

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

# Get an OS directory path.
# Usage: $ make os_path
.PHONY: os_path
os_path:
	@echo $(abspath $(TARGET))

# Get an OS media size.
# Usage: $ make media_size
.PHONY: media_size
media_size:
	@echo $(MEDIA_SIZE)

# Touch the all source files.
.PHONY: touch
touch:
	touch $(call SOURCE_FILES, .)

include $(shell git rev-parse --show-toplevel)/.make/footer.mk


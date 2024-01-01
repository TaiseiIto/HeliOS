# This makefile is called from ../.tmux/run.conf and ../.tmux/debug.conf.
# Don't execute this directly.

QEMU=qemu-system-x86_64
MEMORY_SIZE=-m 1G
LOG_PATH=../qemu.log
LOG=-d int,cpu_reset -D $(LOG_PATH)
NO_REBOOT=--no-reboot
OVMF_CODE_PATH=../../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_CODE.fd
OVMF_CODE=-drive file=$(OVMF_CODE_PATH),format=raw,if=pflash,readonly=on
OVMF_VARS_PATH=../../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_VARS.fd
OVMF_VARS=-drive file=$(OVMF_VARS_PATH),format=raw,if=pflash,readonly=on
OS=-drive file=$(OS_PATH),format=raw,id=$(OS_NAME),if=none -device ide-hd,drive=$(OS_NAME),bootindex=1
SERIAL_LOG_PATH=../serial.log
SERIAL_LOG=-serial stdio
VNC_OPTION=-vnc :0
DEBUG=-S -gdb tcp::$(DEBUG_PORT)

# Run the OS on QEMU.
# Usage: $ make run OS_PATH=<os image file path> OS_NAME=<os name>
.PHONY: run
run:
	$(QEMU) $(MEMORY_SIZE) $(LOG) $(NO_REBOOT) $(OVMF_CODE) $(OVMF_VARS) $(OS) $(SERIAL_LOG) $(VNC_OPTION) | tee $(SERIAL_LOG_PATH)

# Debug the OS on QEMU by GDB.
# Usage: $ make debug OS_PATH=<os image file path> OS_NAME=<os name> DEBUG_PORT=<debug port>
.PHONY: debug
debug:
	$(QEMU) $(MEMORY_SIZE) $(LOG) $(NO_REBOOT) $(OVMF_CODE) $(OVMF_VARS) $(OS) $(SERIAL_LOG) $(VNC_OPTION) $(DEBUG) | tee $(SERIAL_LOG_PATH)

# Stop the OS on QEMU.
# Usage: $ make stop
.PHONY: stop
stop:
	for i in $$(ps ax | grep $(QEMU) | grep -v grep | awk '{print $$1}'); do kill $$i; done


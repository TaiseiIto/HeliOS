# This makefile is called from ../.tmux/run.conf and ../.tmux/debug.conf.
# Don't execute this directly.

QEMU=qemu-system-x86_64
LOG=-d int,cpu_reset -D $(LOG_PATH)
LOG_PATH=../qemu.log
MEMORY_SIZE=-m 1G
MONITOR=-monitor telnet::$(TELNET_PORT),server,nowait
NO_REBOOT=--no-reboot
OS=-drive file=$(OS_PATH),format=raw,id=$(OS_NAME),if=none -device ide-hd,drive=$(OS_NAME),bootindex=1
OVMF_CODE=-drive file=$(OVMF_CODE_PATH),format=raw,if=pflash,readonly=on
OVMF_CODE_PATH=../../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_CODE.fd
OVMF_VARS=-drive file=$(OVMF_VARS_PATH),format=raw,if=pflash,readonly=on
OVMF_VARS_PATH=../../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_VARS.fd
SERIAL_LOG=-serial stdio
SERIAL_LOG_PATH=../serial.log
VNC=-vnc :0
DEBUG=-S -gdb tcp::$(DEBUG_PORT)

# Run the OS on QEMU.
# Usage: $ make run OS_PATH=<os image file path> OS_NAME=<os name>
.PHONY: run
run:
	-$(QEMU) $(LOG) $(MEMORY_SIZE) $(MONITOR) $(NO_REBOOT) $(OS) $(OVMF_CODE) $(OVMF_VARS) $(SERIAL_LOG) $(VNC) | tee $(SERIAL_LOG_PATH)

# Debug the OS on QEMU by GDB.
# Usage: $ make debug OS_PATH=<os image file path> OS_NAME=<os name> DEBUG_PORT=<debug port>
.PHONY: debug
debug:
	-$(QEMU) $(LOG) $(MEMORY_SIZE) $(MONITOR) $(NO_REBOOT) $(OS) $(OVMF_CODE) $(OVMF_VARS) $(SERIAL_LOG) $(VNC) $(DEBUG) | tee $(SERIAL_LOG_PATH)

# Stop the OS on QEMU.
# Usage: $ make stop
.PHONY: stop
stop:
	-echo quit | nc localhost $(TELNET_PORT)
	-tmux kill-server


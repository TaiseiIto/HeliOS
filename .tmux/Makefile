# This makefile is called from ../Makefile
# Don't execute this directly.

# Run the OS on QEMU.
# Usage: make run
.PHONY: run
run:
	-tmux new-session \; source-file run.conf

# Debug the OS on QEMU by GDB
# Usage: make debug
.PHONY: debug
debug:
	-tmux new-session \; source-file debug.conf

# Debug QEMU by GDB
# Usage: make debug_qemu
.PHONY: debug_qemu
debug_qemu:
	-tmux new-session \; source-file debug_qemu.conf
	
# Debug QEMU without HPET by GDB
# Usage: make debug_qemu_without_hpet
.PHONY: debug_qemu_without_hpet
debug_qemu_without_hpet:
	-tmux new-session \; source-file debug_qemu_without_hpet.conf
	
# Stop the OS on QEMU.
# Usage: make stop
.PHONY: stop
stop:
	-make stop_on_tmux -C ..


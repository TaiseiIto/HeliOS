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
	
# Stop the OS on QEMU.
# Usage: make stop
.PHONY: stop
stop:
	-make stop_on_tmux -C ..


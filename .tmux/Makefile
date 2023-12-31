# This makefile is called from ../Makefile
# Don't execute this directly.

# Run the OS on QEMU.
# Usage: make run
.PHONY: run
run:
	tmux new-session \; source-file run.conf

# Debug the OS on QEMU by GDB
# Usage: make debug
.PHONY: debug
debug:
	tmux new-session \; source-file debug.conf
	
# Stop the OS on QEMU.
# Usage: make stop
.PHONY: stop
stop:
	make stop -C ..
	for i in $$(ps ax | grep tmux | grep -v grep | grep -v make | awk '{print $$1}'); do kill $$i; done


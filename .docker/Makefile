# This makefile is called from ../Makefile
# Don't execute it directly.

# Build and enter a development environment as a Docker container.
# Usage: make DEVELOPER=developer REPOSITORY=repository
environment:
	./environment.sh $(DEVELOPER) $(REPOSITORY)

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
permission:
	./permission.sh $(SSHKEY) $(GPGKEY)


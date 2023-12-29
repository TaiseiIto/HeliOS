# This makefile is called from ../Makefile
# Don't execute it directly.

# Build and enter a development environment as a Docker container.
environment:
	./environment.sh

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage+ $make permission GITHUB=/path/to/ssh/key GITGPG=/path/to/.gnupg
permission:
	./permission.sh $(GITHUB) $(GITGPG)


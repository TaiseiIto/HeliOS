# Build and enter a development environment as a Docker container.
# Usage: $ make build_environment
.PHONY: build_environment
build_environment:
	make build -C .docker

# Delete a development environment.
# Usage: $ make delete_environment
.PHONY: delete_environment
delete_environment:
	make delete -C .docker

# Rebuild and enter a development environment.
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


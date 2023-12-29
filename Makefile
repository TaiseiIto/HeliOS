# Prepare a development environment as a Docker container.
# Usage: $ make environment
environment:
	make environment -C .docker

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
permission:
	make permission -C .docker SSHKEY=$(realpath $(SSHKEY)) GPGKEY=$(realpath $(GPGKEY))


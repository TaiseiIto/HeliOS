DOMAIN=$(shell git config --get remote.origin.url | awk -F '[@:/]' '{print $$(NF-2)}')
DEVELOPER=$(shell git config --get remote.origin.url | awk -F '[:/]' '{print $$(NF-1)}')
REPOSITORY=$(shell git config --get remote.origin.url | awk -F '[/.]' '{print $$(NF-1)}')

all:
	echo $(DOMAIN)
	echo $(DEVELOPER)
	echo $(REPOSITORY)

# Prepare a development environment as a Docker container.
# Usage: $ make environment
environment:
	make environment -C .docker DEVELOPER=$(DEVELOPER) REPOSITORY=$(REPOSITORY)

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
permission:
	make permission -C .docker SSHKEY=$(realpath $(SSHKEY)) GPGKEY=$(realpath $(GPGKEY))


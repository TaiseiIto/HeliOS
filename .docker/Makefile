# This makefile is called from ../Makefile
# Don't execute it directly.

REPOSITORY=$(shell git config --get remote.origin.url)
DOMAIN=$(shell echo $(REPOSITORY) | awk -F '[/:@]' '{print $$(NF-2)}')
DEVELOPER=$(shell echo $(REPOSITORY) | awk -F '[/:]' '{print $$(NF-1)}')
PRODUCT=$(shell echo $(REPOSITORY) | awk -F '[./]' '{print $$(NF-1)}')
IMAGE=$(shell echo $(PRODUCT) | awk '{print tolower($$0)}')
CONTAINER=$(IMAGE)

all:
	echo $(DOMAIN)
	echo $(DEVELOPER)
	echo $(PRODUCT)
	echo $(IMAGE)
	echo $(CONTAINER)

# Build and enter a development environment as a Docker container.
# Usage: make
environment:
	./environment.sh

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
permission:
	./permission.sh $(SSHKEY) $(GPGKEY)


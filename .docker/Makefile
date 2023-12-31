# This makefile is called from ../Makefile
# Don't execute it directly.

REPOSITORY=$(shell git config --get remote.origin.url)
DOMAIN=$(shell echo $(REPOSITORY) | awk -F '[/:@]' '{print $$(NF-2)}')
DEVELOPER=$(shell echo $(REPOSITORY) | awk -F '[/:]' '{print $$(NF-1)}')
PRODUCT=$(shell echo $(REPOSITORY) | awk -F '[./]' '{print $$(NF-1)}')
IMAGE=$(shell echo $(PRODUCT) | awk '{print tolower($$0)}')
CONTAINER=$(IMAGE)

# Build and enter development environment as a Docker container.
# Usage: make build VNC_PORT=<vnc port>
.PHONY: build
build:
	./build.sh $(DOMAIN) $(DEVELOPER) $(PRODUCT) $(IMAGE) $(CONTAINER) $(VNC_PORT)

# Delete development environment.
# Usage: make delete
.PHONY: delete
delete:
	./delete.sh $(IMAGE) $(CONTAINER)

# Rebuild and enter development environment.
.PHONY: rebuild
rebuild:
	make delete
	make build

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
.PHONY: permission
permission:
	./permit.sh $(CONTAINER) $(DOMAIN) $(DEVELOPER) $(PRODUCT) $(SSHKEY) $(GPGKEY)


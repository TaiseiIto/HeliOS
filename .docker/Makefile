# This makefile is called from ../Makefile
# Don't execute it directly.

# This makefile should be able to analyze both HTTPS URL and SSH URL.
REPOSITORY=$(shell git config --get remote.origin.url)
DOMAIN=$(shell echo $(REPOSITORY) | awk -F '[/:@]' '{print $$(NF-1)}')
DEVELOPER=$(shell echo $(REPOSITORY) | awk -F '[/:]' '{print $$(NF-1)}')
PRODUCT=$(shell echo $(REPOSITORY) | awk -F '[./]' '{print $$(NF-1)}')
IMAGE=$(shell echo $(PRODUCT) | awk '{print tolower($$0)}')
CONTAINER=$(IMAGE)

# Build and enter development environment as a Docker container.
# Usage: make build VNC_PORT=<vnc port> DEBUG_PORT=<debug port>
.PHONY: build
build:
	./build.sh attach $(DOMAIN) $(DEVELOPER) $(PRODUCT) $(IMAGE) $(CONTAINER) $(VNC_PORT) $(DEBUG_PORT)

# Build the OS on GitHub Actions.
# Usage: make build_on_github VNC_PORT=<vnc port> DEBUG_PORT=<debug port>
.PHONY: build_on_github
build_on_github:
	./build.sh build_on_github $(DOMAIN) $(DEVELOPER) $(PRODUCT) $(IMAGE) $(CONTAINER) $(VNC_PORT) $(DEBUG_PORT)

# Delete development environment.
# Usage: make delete
.PHONY: delete
delete:
	./delete.sh $(IMAGE) $(CONTAINER)

# Rebuild and enter development environment.
# Usage: make rebuild VNC_PORT=<vnc port> DEBUG_PORT=<debug port>
.PHONY: rebuild
rebuild:
	make delete
	make build VNC_PORT=$(VNC_PORT) DEBUG_PORT=$(DEBUG_PORT)

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage: $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
.PHONY: permission
permission:
	./permit.sh $(CONTAINER) $(DOMAIN) $(DEVELOPER) $(PRODUCT) $(SSHKEY) $(GPGKEY)


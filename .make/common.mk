SHELL=/bin/bash
PRODUCT=$(shell basename $$(pwd))

define source_files
	$(shell git ls-files -- $(1) && git ls-files --others --exclude-standard -- $(1))
endef


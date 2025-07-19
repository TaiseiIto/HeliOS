SHELL=/bin/bash
PRODUCT=$(notdir $(shell pwd))
SOURCE_FILES=$(shell git ls-files -- $(1) && git ls-files --others --exclude-standard -- $(1))


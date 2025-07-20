SHELL=/bin/bash
PRODUCT=$(notdir $(abspath .))
SOURCE_FILES=$(shell git ls-files -- $1; git ls-files --others --exclude-standard -- $1)


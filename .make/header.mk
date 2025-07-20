SHELL=/bin/bash
PRODUCT=$(notdir $(abspath .))
SOURCE_FILES=$(shell git ls-files -- $1; git ls-files --others --exclude-standard -- $1)
SUB_TARGET=$(shell make target -C $1 -s)


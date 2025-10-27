SHELL=/bin/bash
PRODUCT=$(notdir $(abspath .))
SOURCE_FILES=$(shell comm -23 <(git ls-files --cached --exclude-standard --others -- $1) <(git ls-files --deleted -- $1))
SUB_TARGET=$(shell make target -C $1 -s)


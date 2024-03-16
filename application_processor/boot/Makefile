SHELL=/bin/bash
TARGET=loader.bin
SOURCES=$(shell ls *.s)
OBJECTS=$(SOURCES:.s=.o)
LINKER_SCRIPT=linker_script.ld
MAP=memory.map
DISASSEMBLE=disassemble.txt

.PHONY: all
all: $(TARGET)

.PHONY: disassemble
disassemble: $(DISASSEMBLE)
	cat $^

$(TARGET): $(OBJECTS) $(shell find . -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	ld $(OBJECTS) -T $(LINKER_SCRIPT) -Map $(MAP) -o $@

$(DISASSEMBLE): $(TARGET)
	objdump -D -b binary -mi8086 -Maddr16,data16 $^ > $@

$(OBJECTS): $(@:.o=.s)

%.o: %.s
	gcc $^ -c -nostdlib -Wall -Wextra -o $@


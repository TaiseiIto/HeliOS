SHELL=/bin/bash
TARGET=loader.bin
SOURCES=$(shell ls *.s)
OBJECTS=$(SOURCES:.s=.o)
LINKER_SCRIPT=linker_script.ld
MAP=memory.map
DISASSEMBLE16=disassemble16.txt
DISASSEMBLE32=disassemble32.txt
DISASSEMBLE64=disassemble64.txt

.PHONY: all
all: $(TARGET) $(DISASSEMBLE16) $(DISASSEMBLE32) $(DISASSEMBLE64)

$(TARGET): $(OBJECTS) $(shell find . -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	ld $(OBJECTS) -T $(LINKER_SCRIPT) -Map $(MAP) -o $@

$(DISASSEMBLE16): $(TARGET)
	objdump -D -b binary -m i8086 -Maddr16,data16 $^ > $@

$(DISASSEMBLE32): $(TARGET)
	objdump -D -b binary -m i386 $^ > $@

$(DISASSEMBLE64): $(TARGET)
	objdump -D -b binary -m i386:x86-64 $^ > $@

$(OBJECTS): $(@:.o=.s)

%.o: %.s
	gcc $^ -c -nostdlib -Wall -Wextra -o $@

# Get a target file path.
.PHONY: target
target:
	@echo $(abspath $(TARGET))


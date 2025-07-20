include $(shell git rev-parse --show-toplevel)/.make/header.mk

TARGET=loader.bin
SOURCES=$(wildcard *.s)
OBJECTS=$(SOURCES:.s=.o)
LINKER_SCRIPT=linker_script.ld
MAP=memory.map
DISASSEMBLE16=disassemble16.txt
DISASSEMBLE32=disassemble32.txt
DISASSEMBLE64=disassemble64.txt

.PHONY: all
all: $(TARGET) $(DISASSEMBLE16) $(DISASSEMBLE32) $(DISASSEMBLE64)

$(TARGET): $(OBJECTS) $(call SOURCE_FILES, .)
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

include $(shell git rev-parse --show-toplevel)/.make/footer.mk


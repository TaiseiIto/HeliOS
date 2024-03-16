TARGET=loader.bin
LINKER_SCRIPT=linker_script.ld
MAP=memory.map
SOURCES=$(shell ls *.s)
OBJECTS=$(SOURCES:.s=.o)

$(TARGET): $(shell find . -type f | grep -v ^.*/\.git/.*$ | grep -vf <(git ls-files --exclude-standard --ignored -o))
	ld $(OBJECTS) -T $(LINKER_SCRIPT) -Map $(MAP) -o $@

%.o: $(@:.o=.s)
	gcc $^ -c -nostdlib -Wall -Wextra -o $@


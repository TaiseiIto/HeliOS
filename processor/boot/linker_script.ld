OUTPUT_FORMAT("binary");

BASE = 0x2000;

SECTIONS {
	. = BASE;
	.text : {
		main.o(.text)
	}
	.data : {
		main.o(.data)
	}
	/DISCARD/ : {
		*(.eh_frame)
		*(.note.gnu.property)
	}
}


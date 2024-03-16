OUTPUT_FORMAT("binary");

SECTIONS {
	. = 0x1000;
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


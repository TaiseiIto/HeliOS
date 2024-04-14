OUTPUT_FORMAT("binary");

BASE = 0x1000;

SECTIONS {
	. = BASE;
	.text : {
		main.o(*)
	}
	/DISCARD/ : {
		*(.eh_frame)
		*(.note.gnu.property)
	}
}


	.text
	.code16
main:
0:
	movw	0x7000,	%ss
1:
	hlt
	jmp	1b


	.set	segment_length,	0x00010000
	.set	segment_shift,	4
	.set	stack_floor,	0x00080000
	.set	stack_segment,	(stack_floor - segment_length) >> segment_shift
	.text
	.code16
main:
0:	# Initialize the general registers.
	xorw	%ax,	%ax
	movw	%ax,	%bx
	movw	%ax,	%cx
	movw	%ax,	%dx
	movw	%ax,	%si
	movw	%ax,	%di
	movw	%ax,	%sp
	movw	%ax,	%bp
1:	# Initialize the segment registers.
	movw	%ax,	%ds
	movw	%ax,	%es
	movw	%ax,	%fs
	movw	%ax,	%gs
	movw	stack_segment,	%ss
2:	# Halt loop
	hlt
	jmp	2b


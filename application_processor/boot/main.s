	.set	SEGMENT_LENGTH,	0x00010000
	.set	SEGMENT_SHIFT,	4
	.set	STACK_FLOOR,	0x00080000
	.set	STACK_SEGMENT,	(STACK_FLOOR - SEGMENT_LENGTH) >> SEGMENT_SHIFT

	.set	COM3,	0x03e8
	.set	COM3_TRANSMITTER_HOLDING_BUFFER,	COM3 + 0x0000
	.set	COM3_DIVISOR_LATCH_LOW_BYTE,		COM3 + 0x0000
	.set	COM3_DIVISOR_LATCH_HIGH_BYTE,		COM3 + 0x0001
	.set	COM3_INTERRUPT_ENABLE,			COM3 + 0x0001
	.set	COM3_FIFO_CONTROL,			COM3 + 0x0002
	.set	COM3_LINE_CONTROL,			COM3 + 0x0003
	.set	COM3_MODEM_CONTROL,			COM3 + 0x0004
	.set	COM3_LINE_STATUS,			COM3 + 0x0005

	.text
	.code16
main:	# IP == 0x1000
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
	movw	STACK_SEGMENT,	%ss
2:	# Create a main stack frame.
	enter	$0x0000,	$0x00
	call	initialize_com3
	leave
3:	# Halt loop
	hlt
	jmp	3b

initialize_com3:
0:
	enter	$0,	$0
	# Disable all interrupts.
	mov	$0x00,	%al
	mov	COM3_INTERRUPT_ENABLE,	%dx
	outb	%al,	%dx
	leave
	ret


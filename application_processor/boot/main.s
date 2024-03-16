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
	.set	COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH,	0x80
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

disable_com3_interrupts:
0:
	enter	$0x0000,	$0x00
	xorb	%al,	%al
	mov	COM3_INTERRUPT_ENABLE,	%dx
	outb	%al,	%dx
	leave
	ret

enable_com3_divisor_access_latch:
0:
	enter	$0x0000,	$0x00
	mov	COM3_LINE_CONTROL,	%dx
	inb	%dx,	%al
	testb	COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH,	%al
	jnz	2f
1:	# If divisor access latch is disabled, enable it.
	orb	COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH, %al
	outb	%al,	%dx
2:	# If divisor access latch is enabled, do nothing.
	leave
	ret

initialize_com3:
0:
	enter	$0x0000,	$0x00
	call	disable_com3_interrupts
	call	enable_com3_divisor_access_latch
	leave
	ret


# Calling convention = System V i386
# Return value: ax, dx
# Parameters: stack
# Scratch registers: ax, cx, dx
# Preserved registers: bx, si, di, bp, sp

	.set	SEGMENT_LENGTH,	0x00010000
	.set	SEGMENT_SHIFT,	4
	.set	STACK_FLOOR,	0x00010000
	.set	STACK_SEGMENT,	(STACK_FLOOR - SEGMENT_LENGTH) >> SEGMENT_SHIFT

	.set	COM3,	0x03e8
	.set	COM3_BAUD_RATE,	9600
	.set	COM3_FREQUENCY,	115200
	.set	COM3_BAUD_RATE_DIVISOR,	COM3_FREQUENCY / COM3_BAUD_RATE
	.set	COM3_TRANSMITTER_HOLDING_BUFFER,	COM3 + 0x0000
	.set	COM3_DIVISOR_LATCH_LOW_BYTE,		COM3 + 0x0000
	.set	COM3_DIVISOR_LATCH_HIGH_BYTE,		COM3 + 0x0001
	.set	COM3_INTERRUPT_ENABLE,			COM3 + 0x0001
	.set	COM3_FIFO_CONTROL,			COM3 + 0x0002
	.set	COM3_LINE_CONTROL,			COM3 + 0x0003
	.set	COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH,	0x80
	.set	COM3_MODEM_CONTROL,			COM3 + 0x0004
	.set	COM3_LINE_STATUS,			COM3 + 0x0005

	.data
hello_world:
	.string	"Hello, World!\n"

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
	pushw	hello_world
	call	puts
	leave
3:	# Halt loop
	hlt
	jmp	3b

com3_can_send:
0:
	enter	$0x0000,	$0x00
	call	read_com3_line_status
	andw	$0x0020,	%ax
	leave
	ret

disable_com3_divisor_access_latch:
0:
	enter	$0x0000,	$0x00
	call	read_com3_line_control
	testb	COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH,	%al
	jz	2f
1:	# If divisor access latch is enabled, disable it.
	andb	~COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH,	%al
	pushw	%ax
	call	write_com3_line_control
2:	# If divisor access latch is disabled, do nothing.
	leave
	ret

enable_com3_divisor_access_latch:
0:
	enter	$0x0000,	$0x00
	call	read_com3_line_control
	testb	COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH,	%al
	jnz	2f
1:	# If divisor access latch is disabled, enable it.
	orb	COM3_LINE_CONTROL_DIVISOR_ACCESS_LATCH, %al
	pushw	%ax
	call	write_com3_line_control
2:	# If divisor access latch is enabled, do nothing.
	leave
	ret

initialize_com3:
0:
	enter	$0x0000,	$0x00
1:	# Disable all interrupts.
	push	$0x0000
	call	write_com3_interrupt_enable
2:	# Set baud rate divisor.
	pushw	COM3_BAUD_RATE_DIVISOR
	call	write_com3_baud_rate_divisor
3:	# 8 bits, no parity, one stop bit
	pushw	$0x0003
	call	write_com3_line_control
4:	# Enable FIFO, clear them, with 14-byte threshold
	pushw	$0x00c7
	call	write_com3_fifo_control
5:	# IRQs enabled, RTS/DSR set
	pushw	$0x000b
	call	write_com3_modem_control
6:	# If serial is not faulty set it in normal operation mode
        # (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
	pushw	$0x000f
	call	write_com3_modem_control
7:
	leave
	ret

putchar:
0:
	enter	$0x0000,	$0x00
1:	# Check if COM3 can send a byte.
	call	com3_can_send
	testb	%al,	%al
	jz	1b
2:	# Disable COM3 divisor access latch
	call	disable_com3_divisor_access_latch
3:	# Send a byte
	movb	0x04(%bp),	%al
	pushw	%ax
	call	write_com3_transmitter_holding_buffer
4:
	leave
	ret

puts:
0:
	enter	$0x0004,	$0x00
	movw	%si,	-0x2(%bp)
	movw	0x04(%bp),	%si
1:
	movb	(%si),	%al
	testb	%al,	%al
	jz	2f
	movb	%al,	-0x4(%bp)
	call	putchar
	incw	%si
	jmp	1b
2:
	movw	-0x2(%bp),	%si
	leave
	ret

read_com3_line_control:
0:
	enter	$0x0000,	$0x00
	movw	COM3_LINE_CONTROL,	%dx
	inb	%dx,	%al
	leave
	ret

read_com3_line_status:
0:
	enter	$0x0000,	$0x00
	movw	COM3_LINE_STATUS,	%dx
	inb	%dx,	%al
	leave
	ret

write_com3_baud_rate_divisor:
0:
	enter	$0x0000,	$0x00
	movw	0x04(%bp),	%bx
	pushw	%bx
	call	write_com3_divisor_latch_low_byte
	movb	%bh,	%bl
	pushw	%bx
	call	write_com3_divisor_latch_high_byte
	leave
	ret

write_com3_divisor_latch_high_byte:
0:
	enter	$0x0000,	$0x00
	call	enable_com3_divisor_access_latch
	movb	0x04(%bp),	%al
	movw	COM3_DIVISOR_LATCH_HIGH_BYTE,	%dx
	outb	%al,	%dx
	leave
	ret

write_com3_divisor_latch_low_byte:
0:
	enter	$0x0000,	$0x00
	call	enable_com3_divisor_access_latch
	movb	0x04(%bp),	%al
	movw	COM3_DIVISOR_LATCH_LOW_BYTE,	%dx
	outb	%al,	%dx
	leave
	ret

write_com3_fifo_control:
0:
	enter	$0x0000,	$0x00
	movb	0x04(%bp),	%al
	movw	COM3_FIFO_CONTROL,	%dx
	outb	%al,	%dx
	leave
	ret

write_com3_interrupt_enable:
0:
	enter	$0x0000,	$0x00
	movb	0x04(%bp),	%al
	movw	COM3_INTERRUPT_ENABLE,	%dx
	outb	%al,	%dx
	leave
	ret

write_com3_line_control:
0:
	enter	$0x0000,	$0x00
	movb	0x04(%bp),	%al
	movw	COM3_LINE_CONTROL,	%dx
	outb	%al,	%dx
	leave
	ret

write_com3_modem_control:
0:
	enter	$0x0000,	$0x00
	movb	0x04(%bp),	%al
	movw	COM3_MODEM_CONTROL,	%dx
	outb	%al,	%dx
	leave
	ret

write_com3_transmitter_holding_buffer:
0:
	enter	$0x0000,	$0x00
	movb	0x04(%bp),	%al
	movw	COM3_TRANSMITTER_HOLDING_BUFFER,	%dx
	outb	%al,	%dx
	leave
	ret


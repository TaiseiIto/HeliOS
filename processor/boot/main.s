# Calling convention = System V i386
# Return value: ax, dx
# Parameters: stack
# Scratch registers: ax, cx, dx
# Preserved registers: bx, si, di, bp, sp

	.set	SEGMENT_LENGTH,	0x00010000
	.set	SEGMENT_SHIFT,	4
	.set	STACK_FLOOR,	0x00010000
	.set	STACK_SEGMENT,	(STACK_FLOOR - SEGMENT_LENGTH) >> SEGMENT_SHIFT

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
2:	# A main function
	enter	$0x0000,	$0x00
	pushw	%di
	leaw	log_start,	%dx
	leaw	log_end_pointer,	%di
	movw	%dx,	(%di)
	movw	message,	%dx
	pushw	%dx
	call	puts
	addw	$0x0002,	%sp
	popw	%di
	leave
3:	# Halt loop
	hlt
	jmp	3b

putchar:
0:
	enter	$0x0000,	$0x00
	pushw	%di
	movw	(log_end_pointer),	%di
	movb	0x04(%bp),	%dl
	movb	%dl,	(%di)
	incw	%di
	movw	%di,	(log_end_pointer)
	popw	%di
	leave
	ret

puts:
0:
	enter	$0x0000,	$0x00
	pushw	%si
	movw	0x04(%bp),	%si
	xorb	%dh,	%dh
1:
	movb	(%si),	%dl
	test	%dl,	%dl
	jz	2f
	pushw	%dx
	call	putchar
	addw	$0x0002,	%sp
	incw	%si
	jmp	1b
2:
	popw	%si
	leave
	ret

	.data
message:
	.string	"Hello from an application processor!\n"
log_end_pointer:
	.word log_start
log_start:



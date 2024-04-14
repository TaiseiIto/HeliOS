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
0:	# Disable interrupts
	cli
	# Initialize the general registers.
	xorw	%ax,	%ax
	movw	%ax,	%bx
	movw	%ax,	%cx
	movw	%ax,	%dx
	movw	%ax,	%si
	movw	%ax,	%di
	movw	%ax,	%sp
	movw	%ax,	%bp
	# Initialize the segment registers.
	movw	%ax,	%ds	
	movw	%ax,	%es	
	movw	%ax,	%fs	
	movw	%ax,	%gs	
	movw	STACK_SEGMENT,	%ss	
	# A main function
	enter	$0x0000,	$0x00
	pushw	%di
	# Set log_end_pointer
	leaw	log_start,	%dx
	leaw	log_end_pointer,	%di
	movw	%dx,	(%di)
	# Print a message
	leaw	message,	%dx
	pushw	%dx
	call	puts
	add	$0x0002,	%sp
	# Leave a main function
	popw	%di
	leave
1:	# Halt loop
	hlt
	jmp	1b

putchar:
0:
	enter	$0x0000,	$0x00
	pushw	%di
	movw	log_end_pointer,	%di
	movb	0x04(%bp),	%dl
	movb	%dl,	(%di)
	incw	%di
	movw	%di,	log_end_pointer
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
	.align	16
gdt:
	# [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.4.5 Segment Descriptors, Figure 3-8. Segment Descriptor
segment_descriptor_null:
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_32bit_code:
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_32bit_data:
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_kernel_code:
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_kernel_data:
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_application_data:
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_application_code:
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
message:
	.string	"Hello from an application processor!\n"
log_end_pointer:
	.word	log_start
log_start:



	.set	SEGMENT_LENGTH,	0x00010000
	.set	SEGMENT_SHIFT,	4
	.set	STACK_FLOOR,	0x00010000
	.set	STACK_SEGMENT,	(STACK_FLOOR - SEGMENT_LENGTH) >> SEGMENT_SHIFT

	.text
	.code16
# Calling convention = System V i386
# Return value: ax, dx
# Parameters: stack
# Scratch registers: ax, cx, dx
# Preserved registers: bx, si, di, bp, sp
main16:	# IP == 0x1000
0:	# Disable interrupts.
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
	# Enter 16bit main function.
	enter	$0x0000,	$0x00
	pushw	%di
	# Set log_end_pointer.
	leaw	log_start,	%dx
	leaw	log_end_pointer,	%di
	movw	%dx,	(%di)
	# Print message16.
	leaw	message16,	%dx
	pushw	%dx
	call	puts16
	addw	$0x0002,	%sp
	# Leave 16bit main function.
	popw	%di
	leave
	# Move to 32bit protected mode.
	lgdt	gdtr
	movl	%cr0,	%edx
	andl	$0x7fffffff,	%edx	# Disable paging,
	orl	$0x00000001,	%edx	# Enable 32bit protected mode.
	movl	%edx,	%cr0
	ljmp	$0x0008,	$main32

putchar16:
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

puts16:
0:
	enter	$0x0000,	$0x00
	pushw	%si
	movw	0x04(%bp),	%si
1:
	movb	(%si),	%dl
	testb	%dl,	%dl
	jz	2f
	pushw	%dx
	call	putchar16
	addw	$0x0002,	%sp
	incw	%si
	jmp	1b
2:
	popw	%si
	leave
	ret

	.code32
# Calling convention = System V i386
# Return value: eax, edx
# Parameters: stack
# Scratch registers: eax, ecx, edx
# Preserved registers: ebx, esi, edi, ebp, esp
main32:
0:	# Set 32bit data segment.
	movw	$0x0010,	%dx
	movw	%dx,	%ds
	movw	%dx,	%es
	movw	%dx,	%fs
	movw	%dx,	%gs
	movw	%dx,	%ss
	leal	STACK_FLOOR,	%ebp
	leal	STACK_FLOOR,	%esp
	# Enter 32bit main function.
	enter	$0x0000,	$0x00
	# Print message32.
	leal	message32,	%edx
	pushl	%edx
	call	puts32
	addl	$0x00000004,	%esp
	# Print bootstrap processor CR3.
	leal	check_cr3_message,	%edx
	pushl	%edx
	call	puts32
	addl	$0x00000004,	%esp
	leal	cr3,	%edx
	pushl	%edx
	call	put_quad_pointer32
	addl	$0x00000004,	%esp
	call	put_new_line32
	# Leave 32bit main function.
	leave
	# Set CR3.
	movl	cr3,	%edx
	movl	%edx,	%cr3
	# Set PAE.
	movl	%cr4,	%edx
	orl	$0x00000020,	%edx
	movl	%edx,	%cr4
	# Set LME and NXE.
	movl	$0xc0000080,	%ecx
	rdmsr
	orl	$0x00000900,	%eax
	wrmsr
	# Set PG.
	movl	%cr0,	%edx
	orl	$0x80000000,	%edx
	mov	%edx,	%cr0
	# Move to 64bit mode.
	ljmp	$0x0018,	$main64

putchar32:
0:
	enter	$0x0000,	$0x00
	pushl	%edi
	movl	log_end_pointer,	%edi
	movb	0x08(%ebp),	%dl
	movb	%dl,	(%edi)
	incl	%edi
	movl	%edi,	log_end_pointer
	popl	%edi
	leave
	ret

puts32:
0:
	enter	$0x0000,	$0x00
	pushl	%esi
	movl	0x08(%ebp),	%esi
1:
	movb	(%esi),	%dl
	testb	%dl,	%dl
	jz	2f
	pushl	%edx
	call	putchar32
	addl	$0x00000004,	%esp
	incl	%esi
	jmp	1b
2:
	popl	%esi
	leave
	ret

put_new_line32:
0:
	enter	$0x0000,	$0x00
	movb	$'\n,	%dl
	pushl	%edx
	call	putchar32
	addl	$0x00000004,	%esp
	leave
	ret

put_nibble32:
0:
	enter	$0x0000,	$0x00
	movb	0x08(%ebp),	%al
	andb	$0x0f,	%al
	movb	%al,	%dl
	subb	$10,	%dl
	jae	2f
1:	# From 0 to 9
	addb	$'0,	%al
	movb	%al,	%dl
	jmp	3f
2:	# From 'a' to 'f'
	addb	$'a,	%dl
3:
	pushl	%edx
	call	putchar32
	addl	$0x00000004,	%esp
	leave
	ret

put_byte32:
0:
	enter	$0x0000,	$0x00
	pushl	%ebx
	movb	0x08(%ebp),	%bl
	movb	%bl,	%dl
	shrb	$0x4,	%dl
	pushl	%edx
	call	put_nibble32
	addl	$0x00000004,	%esp
	pushl	%ebx
	call	put_nibble32
	addl	$0x00000004,	%esp
	popl	%ebx
	leave
	ret

put_word32:
0:
	enter	$0x0000,	$0x00
	pushl	%ebx
	movw	0x08(%ebp),	%bx
	movw	%bx,	%dx
	shrw	$0x8,	%dx
	pushl	%edx
	call	put_byte32
	addl	$0x00000004,	%esp
	pushl	%ebx
	call	put_byte32
	addl	$0x00000004,	%esp
	popl	%ebx
	leave
	ret

put_long32:
0:
	enter	$0x0000,	$0x00
	pushl	%ebx
	movl	0x08(%ebp),	%ebx
	movl	%ebx,	%edx
	shrl	$0x10,	%edx
	pushl	%edx
	call	put_word32
	addl	$0x00000004,	%esp
	pushl	%ebx
	call	put_word32
	addl	$0x00000004,	%esp
	popl	%ebx
	leave
	ret

put_quad32:
0:
	enter	$0x0000,	$0x00
	movl	0x08(%ebp),	%edx
	pushl	%edx
	call	put_long32
	addl	$0x00000004,	%esp
	movl	0x0c(%ebp),	%edx
	pushl	%edx
	call	put_long32
	addl	$0x00000004,	%esp
	leave
	ret

put_quad_pointer32:
0:
	enter	$0x0000,	$0x00
	pushl	%esi
	movl	0x08(%ebp),	%esi
	movl	(%esi),	%edx
	pushl	%edx
	movl	0x04(%esi),	%edx
	pushl	%edx
	call	put_quad32
	addl	$0x00000008,	%esp
	popl	%esi
	leave
	ret

	.code64
# Calling convention = System V x86-64
# Return value: rax, rdx
# Parameters: rdi, rsi, rdx, rcx, r8, r9, stack
# Scratch registers: rax, rcx, rdx, rdi, rsi, r8, r9, r10, r11
# Preserved registers: rbx, rsp, rbp, r12, r13, r14, r15
main64:
0:	# Set 64bit data segment.
	movw	$0x0020,	%dx
	movw	%dx,	%ds
	movw	%dx,	%es
	movw	%dx,	%fs
	movw	%dx,	%gs
	movw	%dx,	%ss
	leaq	STACK_FLOOR,	%rbp
	leaq	STACK_FLOOR,	%rsp
	# Enter 64bit main function.
	enter	$0x0000,	$0x00
	# Print message64.
	leaq	message64,	%rdi
	# Leave 64bit main function.
	leave
1:	# Halt loop.
	hlt
	jmp	1b

putchar64:
0:
	enter	$0x0000,	$0x00
	movb	%dil,	%dl
	movq	log_end_pointer,	%rdi
	movb	%dl,	(%rdi)
	incq	%rdi
	movq	%rdi,	log_end_pointer
	leave
	ret

puts64:
0:
	enter	$0x0000,	$0x00
	movq	%rdi,	%rsi
1:
	movb	(%rsi),	%dil
	testb	%dil,	%dil
	jz	2f
	pushq	%rsi
	call	putchar64
	popq	%rsi
	incq	%rsi
	jmp	1b
2:
	leave
	ret

	.data
	.align	16
gdt_start:
	# [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 3.4.5 Segment Descriptors, Figure 3-8. Segment Descriptor
segment_descriptor_null:			# 0x00
	.word	0x0000	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x00	# Type, S, DPL, P
	.byte	0x00	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_32bit_code:			# 0x08
	.word	0xffff	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x9a	# Type, S, DPL, P
	.byte	0xcf	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_32bit_data:			#0x10
	.word	0xffff	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x92	# Type, S, DPL, P
	.byte	0xcf	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_kernel_code:		# 0x18
	.word	0xffff	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x9a	# Type, S, DPL, P
	.byte	0xaf	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_kernel_data:		# 0x20
	.word	0xffff	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0x92	# Type, S, DPL, P
	.byte	0xcf	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_application_data:	# 0x28
	.word	0xffff	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0xf2	# Type, S, DPL, P
	.byte	0xcf	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
segment_descriptor_64bit_application_code:	# 0x30
	.word	0xffff	# Limit 15:00
	.word	0x0000	# Base 15:00
	.byte	0x00	# Base 23:16
	.byte	0xfa	# Type, S, DPL, P
	.byte	0xaf	# Limit 19:16, AVL, L, D/B, G
	.byte	0x00	# Base 31:24
gdt_end:
gdtr:
	.word	gdt_end - gdt_start - 1
	.long	gdt_start
message16:
	.string	"Hello from an application processor in real mode!\n"
message32:
	.string	"Hello from an application processor in 32bit protected mode!\n"
message64:
	.string	"Hello from an application processor in 64bit mode!\n"
check_cr3_message:
	.string "cr3 = 0x"
log_end_pointer:
	.quad	log_start
	.align	8
cr3:
	.quad	0x0000000000000000
log_start:


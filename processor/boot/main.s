	.set	SEGMENT_LENGTH,	0x00010000
	.set	SEGMENT_SHIFT,	4
	.set	STACK_FLOOR,	0x00010000
	.set	STACK_SEGMENT,	(STACK_FLOOR - SEGMENT_LENGTH) >> SEGMENT_SHIFT
	.set	RFLAGS_ID,	1 << 21

	.text
	.code16
# Calling convention = System V i386
# Return value: ax, dx
# Parameters: stack
# Scratch registers: ax, cx, dx
# Preserved registers: bx, si, di, bp, sp
main16:	# IP == 0x0000
0:	# Disable interrupts.
	cli
	# Initialize the segment registers.
	movw	%cs,	%dx
	movw	%dx,	%ds
	movw	%dx,	%es
	movw	%dx,	%fs
	movw	%dx,	%gs
	movw	boot_argument_ss,	%dx
	movw	%dx,	%ss
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
	# Print CS register.
	leaw	cs_message,	%dx
	pushw	%dx
	call	puts16
	addw	$0x0002,	%sp
	movw	%cs,	%dx
	pushw	%dx
	call	put_word16
	addw	$0x0002,	%sp
	call	put_new_line16
	# Print SS register.
	leaw	ss_message,	%dx
	pushw	%dx
	call	puts16
	addw	$0x0002,	%sp
	movw	%ss,	%dx
	pushw	%dx
	call	put_word16
	addw	$0x0002,	%sp
	call	put_new_line16
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

put_new_line16:
0:
	enter	$0x0000,	$0x00
	movb	$'\n,	%dl
	pushw	%dx
	call	putchar16
	addw	$0x0002,	%sp
	leave
	ret

put_nibble16:
0:
	enter	$0x0000,	$0x00
	pushw	%si
	movb	0x04(%bp),	%al
	andb	$0x0f,	%al
	movb	%al,	%dl
	subb	$10,	%dl
	jae	2f
1:	# From '0' to '9'
	addb	$'0,	%al
	movb	%al,	%dl
	jmp	3f
2:	# From 'a' to 'f'
	addb	$'a,	%dl
3:
	pushw	%dx
	call	putchar16
	addw	$0x0002,	%sp
	leave
	ret

put_byte16:
0:
	enter	$0x0000,	$0x00
	pushw	%bx
	movb	0x04(%bp),	%bl
	movb	%bl,	%dl
	shrb	$0x04,	%dl
	pushw	%dx
	call	put_nibble16
	addw	$0x0002,	%sp
	pushw	%bx
	call	put_nibble16
	addw	$0x0002,	%sp
	popw	%bx
	leave
	ret

put_word16:
0:
	enter	$0x0000,	$0x00
	pushw	%bx
	movw	0x04(%bp),	%bx
	movw	%bx,	%dx
	shrw	$0x08,	%dx
	pushw	%dx
	call	put_byte16
	addw	$0x0002,	%sp
	pushw	%bx
	call	put_byte16
	addw	$0x0002,	%sp
	popw	%bx
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
	leal	cr3_message,	%edx
	pushl	%edx
	call	puts32
	addl	$0x00000004,	%esp
	leal	boot_argument_cr3,	%edx
	pushl	%edx
	call	put_quad_pointer32
	addl	$0x00000004,	%esp
	call	put_new_line32
	# Leave 32bit main function.
	leave
	# Set temporary CR3.
	movl	boot_argument_cr3,	%edx
	andl	$0x00000fff,	%edx
	orl	$temporary_pml4_table,	%edx
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
1:	# From '0' to '9'
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
	call	puts64
	# Set CR3.
	movq	boot_argument_cr3,	%rdx
	movq	%rdx,	%cr3
	# Get IA32_APIC_BASE
	call	get_ia32_apic_base
	movq	%rax,	kernel_argument_ia32_apic_base
	# Print bootstrap processor kernel entry.
	leaq	kernel_entry_message,	%rdi
	call	puts64
	movq	boot_argument_kernel_entry,	%rdi
	call	put_quad64
	call	put_new_line64
	# Print bootstrap processor kernel stack floor.
	leaq	kernel_stack_floor_message,	%rdi
	call	puts64
	movq	boot_argument_kernel_stack_floor,	%rdi
	call	put_quad64
	call	put_new_line64
	# Print BSP heap start
	leaq	bsp_heap_start_message,	%rdi
	call	puts64
	movq	boot_argument_bsp_heap_start,	%rdi
	movq	%rdi,	kernel_argument_bsp_heap_start
	call	put_quad64
	call	put_new_line64
	# Print heap start
	leaq	heap_start_message,	%rdi
	call	puts64
	movq	boot_argument_heap_start,	%rdi
	movq	%rdi,	kernel_argument_heap_start
	call	put_quad64
	call	put_new_line64
	# Print heap size
	leaq	heap_size_message,	%rdi
	call	puts64
	movq	boot_argument_heap_size,	%rdi
	movq	%rdi,	kernel_argument_heap_size
	call	put_quad64
	call	put_new_line64
	# Print receiver
	leaq	message_message,	%rdi
	call	puts64
	movq	boot_argument_receiver,	%rdi
	movq	%rdi,	kernel_argument_receiver
	call	put_quad64
	call	put_new_line64
	# Print sender
	leaq	message_message,	%rdi
	call	puts64
	movq	boot_argument_sender,	%rdi
	movq	%rdi,	kernel_argument_sender
	call	put_quad64
	call	put_new_line64
	# Print my local APIC ID.
	leaq	my_local_apic_id_message,	%rdi
	call	puts64
	call	get_local_apic_id
	movb	%al,	%dil
	call	put_byte64
	call	put_new_line64
	# Print BSP local APIC ID.
	leaq	bsp_local_apic_id_message,	%rdi
	call	puts64
	movb	boot_argument_bsp_local_apic_id,	%dil
	movb	%dil,	kernel_argument_bsp_local_apic_id
	call	put_byte64
	call	put_new_line64
	# Leave 64bit main function.
	leave
	# Jump to the kernel.
	movq	boot_argument_kernel_stack_floor,	%rsp
	leaq	kernel_argument,	%rdi
	call	*boot_argument_kernel_entry

apic_is_supported:
0:
	enter	$0x0000,	$0x00
	call	cpuid_max_eax
	cmpq	$0x0000000000000001,	%rax
	jb	2f
1:	# CPUID EAX=0x00000001 is supported.
	movl	$0x00000001,	%eax
	xorl	%ecx,	%ecx
	pushq	%rbx
	cpuid
	popq	%rbx
	shrq	$0x09,	%rdx
	andq	$0x0000000000000001,	%rdx
	movq	%rdx,	%rax
	jmp	3f
2:	# CPUID EAX=0x00000001 is not supported.
	xorq	%rax,	%rax
3:
	leave
	ret

cpuid_is_supported:
0:
	enter	$0x0000,	$0x00
	call	get_rflags
	orq	$RFLAGS_ID,	%rax
	movq	%rax,	%rdi
	call	set_rflags
	call	get_rflags
	testq	$RFLAGS_ID,	%rax
	jz	2f
	andq	$(~RFLAGS_ID),	%rax
	movq	%rax,	%rdi
	call	set_rflags
	call	get_rflags
	testq	$RFLAGS_ID,	%rax
	jnz	2f
1:	# CPUID is supported.
	movq	$0x0000000000000001,	%rax
	jmp	3f
2:	# CPUID is not supported.
	xorq	%rax,	%rax
3:
	leave
	ret

cpuid_max_eax:
0:
	enter	$0x0000,	$0x00
	call	cpuid_is_supported
	testq	%rax,	%rax
	jz	2f
1:	# CPUID is supported.
	xorl	%eax,	%eax
	xorl	%ecx,	%ecx
	pushq	%rbx
	cpuid
	popq	%rbx
	movq	$0x00000000ffffffff,	%rdx
	andq	%rdx,	%rax
	jmp	3f
2:	# CPUID is not supported.
	call	error
3:
	leave
	ret

error:
0:
	enter	$0x0000,	$0x00
	leaq	error_message,	%rdi
	call	puts64
	call	put_new_line64
	cli
1:
	hlt
	jmp	1b
	leave
	ret

get_ia32_apic_base:
0:
	enter	$0x0000,	$0x00
	call	apic_is_supported
	testq	%rax,	%rax
	jz	2f
1:	# APIC is supported.
	movl	$0x0000001b,	%ecx
	rdmsr
	shlq	$0x20,	%rdx
	movq	$0x00000000ffffffff,	%rcx
	andq	%rcx,	%rax
	addq	%rdx,	%rax
	jmp	3f
2:	# APIC is not supported.
	call	error
3:
	leave
	ret

get_local_apic_base_address:
0:
	enter	$0x0000,	$0x00
	call	get_ia32_apic_base
	movq	$0xfffffffffffff000,	%rdx
	andq	%rdx,	%rax
	leave
	ret

get_local_apic_id:
0:
	enter	$0x0000,	$0x00
	call	get_local_apic_base_address
	movl	0x20(%rax),	%eax
	shrq	$0x18,	%rax
	movq	$0x00000000000000ff,	%rdx
	andq	%rdx,	%rax
	leave
	ret

get_rflags:
0:
	enter	$0x0000,	$0x00
	pushfq
	popq	%rax
	leave
	ret

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

put_new_line64:
0:
	enter	$0x0000,	$0x00
	movb	$'\n,	%dil
	call	putchar64
	leave
	ret

put_nibble64:
0:
	enter	$0x0000,	$0x00
	andb	$0x0f,	%dil
	movb	%dil,	%dl
	subb	$10,	%dl
	jae	2f
1:	# From '0' to '9'
	addb	$'0,	%dil
	jmp	3f
2:	# From 'a' to 'f'
	movb	%dl,	%dil
	addb	$'a,	%dil
3:
	call	putchar64
	leave
	ret

put_byte64:
0:
	enter	$0x0000,	$0x00
	pushq	%rdi
	shrb	$0x4,	%dil
	call	put_nibble64
	popq	%rdi
	call	put_nibble64
	leave
	ret

put_word64:
0:
	enter	$0x0000,	$0x00
	pushq	%rdi
	shrw	$0x8,	%di
	call	put_byte64
	popq	%rdi
	call	put_byte64
	leave
	ret

put_long64:
0:
	enter	$0x0000,	$0x00
	pushq	%rdi
	shrl	$0x10,	%edi
	call	put_word64
	popq	%rdi
	call	put_word64
	leave
	ret

put_quad64:
0:
	enter	$0x0000,	$0x00
	pushq	%rdi
	shrq	$0x20,	%rdi
	call	put_long64
	popq	%rdi
	call	put_long64
	leave
	ret

set_rflags:
0:
	enter	$0x0000,	$0x00
	pushq	%rdi
	popfq
	leave
	ret

	.data
	.align	0x10
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
	.align	0x4
	.word	0x0000
gdtr:
	.word	gdt_end - gdt_start - 1
	.long	gdt_start
bsp_local_apic_id_message:
	.string "BSP local APIC ID = 0x"
cpuid_max_eax_message:
	.string "CPUID max EAX = 0x"
cr3_message:
	.string "CR3 = 0x"
cs_message:
	.string "CS = 0x"
error_message:
	.string	"ERROR!"
bsp_heap_start_message:
	.string "bsp_heap_start = 0x"
heap_start_message:
	.string "heap_start = 0x"
heap_size_message:
	.string "heap_size = 0x"
kernel_entry_message:
	.string "kernel_entry = 0x"
kernel_stack_floor_message:
	.string "kernel_stack_floor = 0x"
message_message:
	.string "message = 0x"
message16:
	.string	"Hello from an application processor in 16bit mode!\n"
message32:
	.string	"Hello from an application processor in 32bit mode!\n"
message64:
	.string	"Hello from an application processor in 64bit mode!\n"
my_local_apic_id_message:
	.string "My local APIC ID = 0x"
ss_message:
	.string "SS = 0x"
log_end_pointer:
	.quad	log_start
	.align	0x8
kernel_argument:	# Argument of ../kernel/src/main.rs
kernel_argument_bsp_heap_start:
	.quad	0x0000000000000000
kernel_argument_heap_start:
	.quad	0x0000000000000000
kernel_argument_heap_size:
	.quad	0x0000000000000000
kernel_argument_ia32_apic_base:
	.quad	0x0000000000000000
kernel_argument_receiver:
	.quad	0x0000000000000000
kernel_argument_sender:
	.quad	0x0000000000000000
kernel_argument_bsp_local_apic_id:
	.byte	0x00
	.align	0x1000
temporary_pml4_table:
	.space	0x1000
	.align	0x8
boot_argument:
boot_argument_cr3:	# Argument of ../../kernel/src/processor/boot.rs
	.quad	0x0000000000000000
boot_argument_kernel_entry:
	.quad	0x0000000000000000
boot_argument_kernel_stack_floor:
	.quad	0x0000000000000000
boot_argument_bsp_heap_start:
	.quad	0x0000000000000000
boot_argument_heap_start:
	.quad	0x0000000000000000
boot_argument_heap_size:
	.quad	0x0000000000000000
boot_argument_sender:
	.quad	0x0000000000000000
boot_argument_receiver:
	.quad	0x0000000000000000
boot_argument_ss:
	.word	0x0000
boot_argument_bsp_local_apic_id:
	.byte	0x00
log_start:


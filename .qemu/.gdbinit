break main
run
# break hw/intc/apic.c: 91 if i * 32 + apic_fls_bit(tab[i]) == 0x21
# break hw/intc/apic.c: 454 if irrv == 0x21
# break hw/intc/apic.c: 754 if intno == 0x21
# break hw/intc/i8259.c: 213 if intno == 0x21
# break hw/i386/x86-cpu.c: 77 if intno == 0x21
# break hw/i386/x86-cpu.c: 86 if intno == 0x21
# break target/i386/tcg/sysemu/seg_helper.c: 209 if intno == 0x21
break do_interrupt_x86_hardirq if intno == 0x21
break do_interrupt_all if intno == 0x21
continue


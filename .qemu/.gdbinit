break main
run

# break do_interrupt_all if intno == 0x99
# continue
# backtrace

# break apic_sipi
# continue
# backtrace

# break ../hw/intc/apic.c:1010
# continue
# backtrace

# AP to BSP
break apic_deliver if vector_num == 0x99
continue
backtrace
break ../target/i386/tcg/sysemu/seg_helper.c:207 if intno == 0x99
break do_interrupt_all if intno == 0x99
continue
backtrace

# BSP to AP
# break apic_deliver if vector_num == 0x9a
# break do_interrupt_all if intno == 0x9a
# continue
# backtrace


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

break apic_set_irq if vector_num == 0x99
continue
backtrace


break main
run

# AP to BSP
# break apic_deliver if vector_num == 0x99
# continue
# break ../hw/intc/apic.c:730
# continue
# backtrace
# print "s->irr"
# p/x s->irr

break apic_set_irq if vector_num == 0x99
continue
backtrace


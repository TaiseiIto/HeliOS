break main
run

# break pit_irq_timer_update
# continue
# continue
# continue
# backtrace
# info symbol s->irq->handler

# break pit_irq_timer
# continue
# backtrace
# info symbol ((PITChannelState*)opaque)->irq->handler

# break qemu_clock_run_all_timers
# continue
# backtrace
# info symbol ((PITChannelState*)main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque)->irq->handler

# print "main_loop_tlg"
# print main_loop_tlg
# print "main_loop_tlg.tl"
# print main_loop_tlg.tl
# print "main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]"
# print main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]
# watch main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]
# continue
# backtrace
# print "main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]"
# print main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]
# print "main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers"
# print main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers
# watch main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers
# continue
# backtrace
# print "main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers"
# print main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers
# print "main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque"
# print main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque
# print "((PITChannelState*)main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque)->irq"
# print ((PITChannelState*)main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque)->irq
# print "((PITChannelState*)main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque)->irq->handler"
# print ((PITChannelState*)main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque)->irq->handler
# print "((PITChannelState*)ts->timer_list->active_timers->opaque)->irq->handler"
# print ((PITChannelState*)ts->timer_list->active_timers->opaque)->irq->handler

# break i8254_pit_init
# continue
# break qom/object.c:1952
# continue
# backtrace
# print "((qemu_irq)new_target)->handler"
# print ((qemu_irq)new_target)->handler

# break i8254_pit_init
# continue
# break qdev_connect_gpio_out_named
# continue
# backtrace
# print "input_pin->handler"
# print input_pin->handler

# break i8254_pit_init
# continue
# backtrace
# print "isa_irq"
# print isa_irq
# print "0 <= isa_irq ? bus->irqs_in[isa_irq]->handler : alt_irq->handler"
# print 0 <= isa_irq ? bus->irqs_in[isa_irq]->handler : alt_irq->handler

# break do_interrupt_all if intno == 0x21
# continue

# break hw/intc/apic.c: 737 if intno == 0x21
# continue
# backtrace
# print "intno"
# p/x intno
# print "s->irr"
# p/x s->irr

# break apic_bus_deliver if vector_num == 0x21
# continue
# backtrace

break ../hw/intc/ioapic.c: 138 if info.vector == 0x21
continue
backtrace
print "info.vector"
p/x info.vector
break ioapic_service if s->irr == 4
continue
backtrace
print "s->irr"
p/x s->irr


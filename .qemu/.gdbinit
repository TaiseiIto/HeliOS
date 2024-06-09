break main
run

# Set the HPET timer 0 comparator register.
# break hw/timer/hpet.c:544

# Copy from the HPET timer 0 comparator retister to period.
# break hw/timer/hpet.c:552

# Start the HPET main counter value increment.
# break hw/timer/hpet.c:596

# Update the HPET timer 0 comparator register.
# break hw/timer/hpet.c:377
# break hw/timer/hpet.c:380

# Interrupt 0x21
# break apic_bus_deliver if vector_num == 0x21
break hw/intc/ioapic.c:106 if info->data == 0x21
# break hw/timer/i8254.c:264 if s->irq->handler == hpet_handle_legacy_irq
# break qdev_init_gpio_in if handler == pit_irq_control || handler == hpet_handle_legacy_irq
continue
break pit_irq_timer_update
continue
continue
continue
continue
continue
continue
continue
continue
continue


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
break do_interrupt_all if intno == 0x21

continue


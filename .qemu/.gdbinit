break main
run

# Set the HPET timer 0 comparator register.
break hw/timer/hpet.c:544

# Copy from the HPET timer 0 comparator retister to period.
break hw/timer/hpet.c:552

# Update the HPET timer 0 comparator register.
break hw/timer/hpet.c:377

continue


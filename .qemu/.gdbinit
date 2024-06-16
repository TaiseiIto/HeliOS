break main
run

# Register gsi_handler
# break hw/i386/pc.c:345

# Register hpet_handle_legacy_irq
break hw/timer/hpet.c:769

# Decide whether to create an HPET
# break hw/i386/pc.c:1209

continue


break main
run

# break pit_irq_timer_update
# continue
# continue
# continue
# backtrace
# info symbol s->irq->handler

break pit_irq_timer
continue
backtrace
info symbol ((PITChannelState*)opaque)->irq->handler


break main
run
break pit_irq_timer
continue
continue
continue
backtrace
info symbol ((PITChannelState*)opaque)->irq->handler


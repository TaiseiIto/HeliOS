break main
run
break pit_irq_control
continue
info symbol ((PITCommonState*)opaque)->channels[0].irq->handler


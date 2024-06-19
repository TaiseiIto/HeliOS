break main
run
break pit_irq_timer_update
continue
info symbol s->irq->handler


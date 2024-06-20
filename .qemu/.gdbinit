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

break qemu_clock_run_all_timers
continue
backtrace
info symbol ((PITChannelState*)main_loop_tlg.tl[QEMU_CLOCK_VIRTUAL]->active_timers->opaque)->irq->handler


break main
run
break ../hw/timer/hpet.c:539
continue
backtrace
print "s->timer[0]"
p/x s->timer[0]


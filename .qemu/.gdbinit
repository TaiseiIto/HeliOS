break main
run
break do_interrupt_all if intno == 0x99
continue
backtrace


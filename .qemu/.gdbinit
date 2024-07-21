break main
run
break memory_region_dispatch_write if mr->ops->write==hpet_ram_write && addr==0x108
continue
backtrace
print "mr->ops->impl.min_access_size"
p/x mr->ops->impl.min_access_size
print "mr->ops->impl.max_access_size"
p/x mr->ops->impl.max_access_size


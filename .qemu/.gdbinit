break main
run
break hpet_handle_legacy_irq
break hw/core/irq.c:73 if handler == hpet_handle_legacy_irq
break hw/core/irq.c:114 if handler == hpet_handle_legacy_irq
break hw/gpio/omap_gpio.c:722 if g_new0(qemu_irq, s->modulecount * 32) == hpet_handle_legacy_irq
break hw/gpio/omap_gpio.c:735 if &s->handler[i * 32] == hpet_handle_legacy_irq
break hw/hyperv/hyperv.c:593  if handler == hpet_handle_legacy_irq
break hw/i386/vapic.c:461 if handlers == hpet_handle_legacy_irq
break hw/input/adb-kbd.c:267 if buf[2] == hpet_handle_legacy_irq
break hw/input/adb-mouse.c:169  if buf[2] == hpet_handle_legacy_irq
break hw/input/virtio-input-hid.c:309 if &virtio_keyboard_handler == hpet_handle_legacy_irq
break hw/input/virtio-input-hid.c:400 if &virtio_mouse_handler == hpet_handle_legacy_irq
break hw/input/virtio-input-hid.c:525 if &virtio_tablet_handler == hpet_handle_legacy_irq
break hw/input/virtio-input-hid.c:607 if &virtio_multitouch_handler == hpet_handle_legacy_irq
break hw/xen/xen-bus.c:894 if handler == hpet_handle_legacy_irq
break subprojects/libvhost-user/libvhost-user.c:1441 if handler == hpet_handle_legacy_irq
break ui/input.c:55 if handler == hpet_handle_legacy_irq
continue


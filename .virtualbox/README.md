# Run HeliOS on VirtualBox

## Create a virtual machine

1. Open VirtualBox.
1. Click `New` to create a new virtual machine.
1. Machine name is `HeliOS`.
1. Machine directory path must not have any space.
1. Don't select ISO image.
1. Machine type is `Other`.
1. Machine version is `Other/Unknown (64-bit)`.
1. Click `Next`.
1. Base memory is 1GiB.
1. The machine has 2 processors.
1. Check `Enable EFI (special OSes only)`.
1. Click `Next`.
1. Select `Do not add a virtual hard disk`.
1. Click `Next`.
1. Click `Finish`.
1. Click `Continue`.

## Set the machine.

### Check the machine settings.

1. Select the machine `HeliOS` and click `Settings`.
1. Check `General -> Basic -> Name` is `HeliOS`.
1. Check `General -> Basic -> Type` is `Other`.
1. Check `General -> Basic -> Version` is `Other/Unknown (64-bit)`.
1. Check `System -> Motherboard -> Base Memory` is 1GiB.
1. Check `System -> Motherboard -> Extended Features -> Enable I/O APIC` is checked.
1. Check `System -> Motherboard -> Extended Features -> Enable EFI (special OSes only)` is checked.

### Add a hard disk.

1. Remove `Storage -> Controller: IDE -> Empty`.
1. Add a hard disk to `Storage -> Controller: IDE`.
1. Click `Create`.
1. Select `VHD (Virtual Hard Disk)`.
1. Click `Next`.
1. Check `Pre-allocate Full Size`.
1. Click `Next`.
1. Check the hard disk name is `.../HeliOS.vhd`.
1. Set the hard disk size 64MiB.
1. Click `Finish`.
1. Click `Choose`.

### Add a serial port.

1. Check `Serial Ports -> Port 1 -> Enable Serial Port`
1. The port number is `COM2`.
1. The port mode is `Raw file`.
1. Set `Path/Address` as the COM2 log file path where you would like.

### Add a USB controller.

1. Check `USB -> Enable SUB Controller`.
1. Select `USB 3.0 (xHCI) Controller`.

### Enable HPET

```
/somewhere$ vboxmanage modifyvm HeliOS --hpet on
```

## Build HeliOS.

```
/somewhere/HeliOS$ make tree
```

## Write HeliOS to the virtual hard disk.

```
/somewhere/HeliOS/.virtualbox$ ./write_vhd.sh
```

## Start HeliOS on the virtual machine.

1. Select the virtual machine on the VirtualBox.
1. Click `Start`.

If the following error occured,

```
VirtualBox can't operate in VMX root mode. Please disable the KVM kernel extension, recompile your kernel and reboot.
```

First, close the docker desktop and wait for stopping QEMU.

```
* $ while pgrep -a qemu; do sleep 1; done
```

Then, disable the KVM modules.

```
* $ for module in $(lsmod | awk '{print $1}' | grep kvm); do sudo rmmod $module; done
```

Now, you can restart HeliOS on the virtual machine.


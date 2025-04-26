# Run HeliOS on VMware

## Create a virtual machine

1. Open VMware Workstation Pro.
1. Click `File -> New Virtual Machine`.
1. Select `Typical (recommended)`.
1. Click `Next`.
1. Select `I will install the operation system later.`
1. Click `Next`.
1. Select guest operation system `Other`.
1. Select Version `Other 64-bit`.
1. Click `Next`.
1. Virtual machine name is `HeliOS`.
1. Note the `location` where the vmx file will be in and the location must not have any space.
1. Click `Next`.
1. The maximum disk size is 1GB.
1. Select `Store virtual disk as a single file`.
1. Click `Next`.
1. Click `Customize Hardware`.
1. Memory is 1GiB.
1. Number of processors is 2.
1. Number of cores per processor is 1.
1. Remove device `New CD/DVD (IDE)`
1. Add device `USB Controller`.
1. Click `Finish`.
1. USB compatibility is `USB 3.1`.
1. Check `Automatically connect new USB devices`.
1. Add device `Serial Port`.
1. Select `Use output file` and enter COM1 log file path.
1. Check `Yield CPU on poll`.
1. Click `Close`.
1. Add device `Serial Port`.
1. Select `Use output file` and enter COM2 log file path.
1. Check `Yield CPU on poll`.
1. Click `Close`.
1. Click `Finish`.
1. Click `Close`.

## Enable UEFI

Add the line below in the bottom of the vmx file of the virtual machine.

```
firmware = "efi"
```

## Build HeliOS.

```
/somewhere/HeliOS$ make tree
```

## Write HeliOS to the virtual hard disk.

```
/somewhere/HeliOS/.vmware$ ./update_vmdk.sh /somewhere/HeliOS.vmx
```

## Start HeliOS on the virtual machine.

1. Select the virtual machine on the VMware Workstation Pro.
1. Click `Start up this guest operation system`.


#!/bin/bash

vmx=$1
virtual_machine_directory=$(dirname $vmx)
vmdk=$virtual_machine_directory/$(grep vmdk $vmx | awk -F '=' '{print $NF}' | sed s/\"//g | sed 's/ //g')
vhd=$(echo $vmdk | sed s/vmdk$/vhd/)
nbd=/dev/nbd0
destination=destination
source=$(make mount_directory -C .. -s)

sudo modprobe nbd max_part=16
qemu-img create -f vpc $vhd 64M
sudo qemu-nbd --format=vpc --connect=$nbd $vhd
sudo mkfs.vfat -v -c -F 32 $nbd
mkdir $destination
sudo mount $nbd $destination
sudo cp -r $source/* $destination
find $destination
sudo umount $destination
sudo rm -rf $destination
sudo qemu-nbd --disconnect $nbd
qemu-img convert -f vpc -O vmdk $vhd $vmdk


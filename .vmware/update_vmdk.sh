#!/bin/bash

vmx=$1
virtual_machine_directory=$(dirname $vmx)
vmdk=$virtual_machine_directory/$(grep vmdk $vmx | awk -F '=' '{print $NF}' | sed s/\"//g | sed 's/ //g')
vhd=$(echo $vmdk | sed s/vmdk$/vhd/)
nbd=/dev/nbd0
destination_path=destination
source_path=$(make mount_directory -C .. -s)
media_size=$(make media_size -C .. -s)

sudo modprobe nbd max_part=16
qemu-img create -f vpc $vhd $media_size
sudo qemu-nbd --format=vpc --connect=$nbd $vhd
while [ $(sudo blockdev --getsize64 $nbd) -eq 0 ]; do
	sleep 0.1
done
sudo mkfs.vfat -v -c -F 32 $nbd
mkdir $destination_path
sudo mount $nbd $destination_path
sudo cp -r $source_path/* $destination_path
find $destination_path
sudo umount $destination_path
sudo rm -rf $destination_path
sudo qemu-nbd --disconnect $nbd
qemu-img convert -f vpc -O vmdk $vhd $vmdk


#!/bin/bash

product=$(git remote get-url origin | awk -F '[/.]' '{print $(NF-1)}')
vhd=$(vboxmanage showvminfo $product | sed -n "s/^\s*Location:\s*\"\(.*\.vhd\)\"$/\1/p")
nbd=/dev/nbd0
destination=destination
source_path=$(make os_path -C .. -s)

sudo modprobe nbd max_part=16
sudo qemu-nbd --format=vpc --connect=$nbd $vhd
while [ $(sudo blockdev --getsize64 $nbd) -eq 0 ]; do
	sleep 0.1
done
sudo mkfs.vfat -v -c -F 32 $nbd
mkdir $destination
sudo mount $nbd $destination
sudo cp -r $source_path/* $destination
sudo umount $destination
sudo rm -rf $destination
sudo qemu-nbd --disconnect $nbd


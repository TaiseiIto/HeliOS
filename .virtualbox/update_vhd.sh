#!/bin/bash

product=$(git remote get-url origin | awk -F '/' '{print $NF}' | awk -F '.' '{print $1}')
vhd=$(vboxmanage showvminfo $product | grep vhd | awk -F ':' '{print $2}' | sed s/\"//g)
nbd=/dev/nbd0
destination=destination
source=$(make mount_directory -C .. -s)

sudo modprobe nbd max_part=16
sudo qemu-nbd --format=vpc --connect=$nbd $vhd
while [ $(sudo blockdev --getsize64 $nbd) -eq 0 ]; do
	sleep 0.1
done
sudo mkfs.vfat -v -c -F 32 $nbd
mkdir $destination
sudo mount $nbd $destination
sudo cp -r $source/* $destination
find $destination
sudo umount $destination
sudo rm -rf $destination
sudo qemu-nbd --disconnect $nbd


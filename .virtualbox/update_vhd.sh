#!/bin/bash

helios=..
vhd=~/VirtualBox\ VMs/HeliOS/HeliOS.vhd
nbd=/dev/nbd0
mnt=mnt

make tree -C $helios
sudo modprobe nbd max_part=16
sudo qemu-nbd --format=vpc --connect=$nbd $vhd
sudo mkfs.vfat -v -c -F 32 $nbd
mkdir $mnt
sudo mount $nbd $mnt
sudo cp -r $helios/HeliOS.mnt/* $mnt
find $mnt
sudo umount $mnt
sudo rm -rf $mnt
sudo qemu-nbd --disconnect $nbd


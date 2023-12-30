#!/bin/sh

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: $ ./permit.sh <conateiner> <domain> <developer> <product> </path/to/ssh/key> </path/to/.gnupg>

container=$1
domain=$2
developer=$3
product=$4
sshkey=$5
gpgkey=$6

docker start $container
home=$(docker exec -i -t helios env | grep HOME | awk -F '=' '{print $2}')
docker cp $sshkey $container:$home/.github/key
docker cp $gpgkey $container:$home/.gnupg
docker exec -i -t $container $home/$product/.git.conf/permit.sh $domain $developer $product
docker stop $container


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
ssh_key_source=$5
gpg_key_source=$6

docker start $container
home=$(docker exec $container env | grep HOME | awk -F '=' '{print $2}')
ssh_key_destination=$home/.github/key
gpg_key_destination=$home/.gnupg
docker cp $ssh_key_source $container:$ssh_key_destination
docker cp $gpg_key_source $container:$gpg_key_destination
docker exec -i -t $container $home/$product/.git.conf/permit.sh $domain $developer $product $ssh_key_destination $gpg_key_destination
docker stop $container


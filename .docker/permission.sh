#!/bin/sh

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: $ ./permission.sh conateiner repository user /path/to/ssh/key /path/to/.gnupg

container=$1
repository=$2
user=$3
sshkey=$4
gpgkey=$5

docker cp $sshkey $container:/$user/.github/key
docker cp $gpgkey $container:/$user/.gnupg
docker start $container
docker exec -i -t $container /$user/$repository/.git.conf/gitconfig.sh
docker stop $container



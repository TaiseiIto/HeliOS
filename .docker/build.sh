#!/bin/bash

# Build development environment as a Docker container.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: ./build.sh <developer> <repository> <image> <container>

developer=$1
repository=$2
image=$3
container=$4

# VNC port to interact with QEMU running on development environment.
vnc_port=5900

# If there is no image named $image, build it.
if [ -z "$(docker images --format {{.Repository}} | grep -x $image)" ]; then
	docker build --build-arg vnc_port=$vnc_port --no-cache -t $image .
fi

# If there is no container named $container, create it.
if [ -z "$(docker ps -a --format {{.Names}} | grep -x $container)" ]; then
	docker create -i -t --name $container $image /bin/bash
fi

# If there is no running container named $container, start it.
if [ -z "$(docker ps --format {{.Names}} | grep -x $container)" ]; then
	docker start $container
fi

# Attach a runnning container $container.
docker attach $container


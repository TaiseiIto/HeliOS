#!/bin/bash

# Build development environment as a Docker container.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: ./build.sh <domain> <developer> <product> <image> <container> <vnc_port> <debug_port>

domain=$1
developer=$2
product=$3
image=$4
container=$5
vnc_port=$6
debug_port=$7
branch=$(git rev-parse --abbrev-ref HEAD)

# If there is no image named $image, build it.
if [ -z "$(docker images --format {{.Repository}} | grep -x $image)" ]; then
	docker build --build-arg domain=$domain --build-arg developer=$developer --build-arg product=$product --build-arg branch=$branch --build-arg debug_port=$debug_port --build-arg vnc_port=$vnc_port --no-cache -t $image .
fi

# If there is no container named $container, create it.
if [ -z "$(docker ps -a --format {{.Names}} | grep -x $container)" ]; then
	docker create -i -t -p $vnc_port:$vnc_port --privileged --name $container $image /bin/bash
fi

# If there is no running container named $container, start it.
if [ -z "$(docker ps --format {{.Names}} | grep -x $container)" ]; then
	docker start $container
fi

# Attach a runnning container $container.
docker attach $container


#!/bin/bash

# Prepare a development environment as a Docker container.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: ./environment.sh developer repository

# Docker image name and Docker container name.
developer=$1
repository=$2
image=$name
container=$name

# A branch name clone into the development environment.
branch=$(git rev-parse --abbrev-ref HEAD)

# VNC port to interact with QEMU running on the development environment.
vnc_port=5900

# If there is no image named "helios", build it.
if [ -z "$(docker images --format {{.Repository}} | grep -x $image)" ]; then
	docker build --build-arg branch=$branch --build-arg vnc_port=$vnc_port --no-cache -t $image .
fi

# If there is no container named "helios", create it.
if [ -z "$(docker ps -a --format {{.Names}} | grep -x $container)" ]; then
	docker create --name $container -i -t $image /bin/bash
fi

# If there is no running container named "helios", start it.
if [ -z "$(docker ps --format {{.Names}} | grep -x $container)" ]; then
	docker start $container
fi

# Attach a runnning container "helios".
docker attach $container


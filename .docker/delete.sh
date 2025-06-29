#!/bin/bash

# Delete development environment built by build.sh.
# This script is called from the Makefile in the same directory.
# Don't execute it directory.
# Usage: ./delete.sh image container

image=$1
container=$2

# If there is a running container named $container, stop it.
if [ -n "$(docker ps --format {{.Names}} --filter name=^$container\$)" ]; then
	docker stop $container
fi

# If there is a container named $container, create it.
if [ -n "$(docker ps -a --format {{.Names}} --filter name=^$container\$)" ]; then
	docker rm $container
fi

# If there is an image named $image, build it.
if [ -n "$(docker images --format {{.Repository}} $image)" ]; then
	docker rmi $image
fi


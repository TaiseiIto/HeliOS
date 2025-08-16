#!/bin/bash

# Build development environment as a Docker container.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Attach usage: ./build.sh attach <domain> <developer> <product> <image> <container> <vnc_port> <debug_port>
# Build on GitHub usage: ./build.sh build_on_github <domain> <developer> <product> <image> <container> <vnc_port> <debug_port>

action=$1
domain=$2
developer=$3
product=$4
image=$5
container=$6
vnc_port=$7
debug_port=$8
branch=$(git rev-parse --abbrev-ref HEAD)

# If there is no image named $image, build it.
if [ -z "$(docker images --format {{.Repository}} $image)" ]; then
	docker build --build-arg domain=$domain --build-arg developer=$developer --build-arg product=$product --build-arg branch=$branch --build-arg debug_port=$debug_port --build-arg vnc_port=$vnc_port -t $image .
fi

# If there is no container named $container, create it.
if [ -z "$(docker ps -a --format {{.Names}} --filter name=^$container\$)" ]; then
	docker create -i -t -p $vnc_port:$vnc_port --privileged --name $container $image /bin/bash
fi

# If there is no running container named $container, start it.
if [ -z "$(docker ps --format {{.Names}} --filter name=^$container\$)" ]; then
	docker start $container
fi

case $action in
	"attach")
		docker attach $container
		;;
	"build_on_github")
		working_directory=$(docker inspect $container --format {{.Config.WorkingDir}})
		docker exec -w $working_directory $container bash -l -c "make tree"
		mount_directory=$(docker exec $container make mount_directory -C $working_directory -s)
		docker stop $container
		docker cp $container:$mount_directory $(dirname $0)/..
		docker rm $container
		docker rmi $image
		;;
	*)
		false
		;;
esac


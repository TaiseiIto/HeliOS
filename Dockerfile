# This is a Dockerfile to prepare a development environment.

FROM ubuntu

# Don't ask stdin anithing to install software automatically.
ENV DEBIAN_FRONTEND=noninteractive

# Install softwares.
RUN apt update

# VNC port
ARG vnc_port
EXPOSE $vnc_port


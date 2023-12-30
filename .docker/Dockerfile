# This is a Dockerfile to prepare a development environment.

FROM ubuntu

# Don't ask stdin anithing to install software automatically.
ENV DEBIAN_FRONTEND=noninteractive

# Update apt.
RUN apt update

# Install git.
RUN apt install git -y

# Clone this repository.
WORKDIR /root
RUN git clone https://github.com/TaiseiIto/HeliOS.git
WORKDIR HeliOS

# Expose VNC port.
ARG vnc_port
EXPOSE $vnc_port


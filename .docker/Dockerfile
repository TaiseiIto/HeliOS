# This is a Dockerfile to prepare development environment.

FROM ubuntu

# Don't ask stdin anithing to install software automatically.
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /root
RUN apt update

# Install git.
RUN apt install git -y
RUN mkdir .ssh
RUN mkdir .github

# Install vim
RUN apt install vim -y

# Clone this repository.
RUN git clone https://github.com/TaiseiIto/HeliOS.git
WORKDIR HeliOS

# Expose VNC port.
ARG vnc_port
EXPOSE $vnc_port


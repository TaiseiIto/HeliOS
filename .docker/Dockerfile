# This is a Dockerfile to prepare development environment.

FROM ubuntu

# Don't ask stdin anithing to install software automatically.
ENV DEBIAN_FRONTEND=noninteractive

WORKDIR /root
RUN apt update

# Install git.
RUN apt install git -y
RUN apt install gnupg -y
RUN mkdir .ssh
RUN mkdir .github

# Install vim
RUN apt install vim -y

# Clone this repository.
ARG domain
ARG developer
ARG product
RUN git clone https://$domain/$developer/$product.git
WORKDIR $product
ARG branch
RUN git checkout $branch

# bash settings
RUN cat .bash/.bashrc >> ../.bashrc

# vim settings
RUN cat .vim/.vimrc >> ../.vimrc

# Expose VNC port.
ARG vnc_port
EXPOSE $vnc_port


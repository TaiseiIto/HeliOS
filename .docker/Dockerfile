# This is a Dockerfile to prepare development environment.
FROM ubuntu:22.04

# Don't ask stdin anithing to install software automatically.
ENV DEBIAN_FRONTEND=noninteractive

# Install softwares.
RUN apt-get update && apt-get upgrade -y && apt-get install -y bison
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential
RUN apt-get update && apt-get upgrade -y && apt-get install -y curl
RUN apt-get update && apt-get upgrade -y && apt-get install -y dejagnu
RUN apt-get update && apt-get upgrade -y && apt-get install -y dosfstools
RUN apt-get update && apt-get upgrade -y && apt-get install -y flex
RUN apt-get update && apt-get upgrade -y && apt-get install -y git
RUN apt-get update && apt-get upgrade -y && apt-get install -y gnupg
RUN apt-get update && apt-get upgrade -y && apt-get install -y iasl
RUN apt-get update && apt-get upgrade -y && apt-get install -y libexpat-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y libglib2.0-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y libgmp-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y libmpfr-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y libpixman-1-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y lld
RUN apt-get update && apt-get upgrade -y && apt-get install -y nasm
RUN apt-get update && apt-get upgrade -y && apt-get install -y netcat-traditional
RUN apt-get update && apt-get upgrade -y && apt-get install -y ninja-build
RUN apt-get update && apt-get upgrade -y && apt-get install -y pkg-config
RUN apt-get update && apt-get upgrade -y && apt-get install -y python3
RUN apt-get update && apt-get upgrade -y && apt-get install -y python3-venv
RUN apt-get update && apt-get upgrade -y && apt-get install -y texinfo
RUN apt-get update && apt-get upgrade -y && apt-get install -y tmux
RUN apt-get update && apt-get upgrade -y && apt-get install -y tzdata
RUN apt-get update && apt-get upgrade -y && apt-get install -y uuid-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y vim

# Install Rust.
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly-2024-01-01-x86_64-unknown-linux-gnu

# Put git settings.
WORKDIR /root
RUN mkdir .ssh
RUN mkdir .github

# Set local time.
RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

# Install GDB.
RUN git clone --branch gdb-14.1-release --depth 1 --recursive --shallow-submodules --single-branch https://sourceware.org/git/binutils-gdb.git gdb
WORKDIR gdb
RUN ./configure --with-expat CFLAGS="-O0 -g -fno-inline" CXXFLAGS="-O0 -g -fno-inline"
RUN make
RUN make install
WORKDIR ..

# Install QEMU.
RUN git clone --branch v8.0.0 --depth 1 --recursive --shallow-submodules --single-branch https://gitlab.com/qemu-project/qemu.git
WORKDIR qemu
RUN ./configure --target-list=x86_64-softmmu CFLAGS="-O0 -g -fno-inline" CXXFLAGS="-O0 -g -fno-inline"
RUN make
RUN make install
WORKDIR roms/edk2
RUN ./OvmfPkg/build.sh -a X64
WORKDIR ../../..

# Clone this repository.
ARG domain
ARG developer
ARG product
RUN git clone https://$domain/$developer/$product.git
WORKDIR $product
ARG branch
RUN git checkout $branch

# Put bash settings.
RUN cat .bash/.bashrc >> ../.bashrc

# Put GDB settings.
ARG debug_port
RUN echo add-auto-load-safe-path $(pwd)/.gdb/.gdbinit >> ../.gdbinit
RUN echo add-auto-load-safe-path $(pwd)/.qemu/.gdbinit >> ../.gdbinit
RUN echo set print elements 0 >> ../.gdbinit
RUN echo set print pretty on >> ../.gdbinit
RUN echo target remote localhost:$debug_port >> .gdb/.gdbinit

# Put tmux settings.
RUN cat .tmux/global.conf >> ../.tmux.conf

# Put vim settings.
RUN cat .vim/.vimrc >> ../.vimrc

# Expose VNC port.
ARG vnc_port
EXPOSE $vnc_port


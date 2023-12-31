# This is a Dockerfile to prepare development environment.

FROM ubuntu

# Don't ask stdin anithing to install software automatically.
ENV DEBIAN_FRONTEND=noninteractive

# Install softwares.
RUN apt update
RUN apt install bison -y
RUN apt install build-essential -y
RUN apt install curl -y
RUN apt install dejagnu -y
RUN apt install flex -y
RUN apt install git -y
RUN apt install gnupg -y
RUN apt install iasl -y
RUN apt install libexpat-dev -y
RUN apt install libglib2.0-dev -y
RUN apt install libgmp-dev -y
RUN apt install libmpfr-dev -y
RUN apt install nasm -y
RUN apt install ninja-build -y
RUN apt install pkg-config -y
RUN apt install python3 -y
RUN apt install python3-venv -y
RUN apt install texinfo -y
RUN apt install tmux -y
RUN apt install tzdata -y
RUN apt install uuid-dev -y
RUN apt install vim -y

# Install Rust.
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y

# Install GDB.
RUN git clone --branch master --depth 1 --recursive --shallow-submodules --single-branch https://sourceware.org/git/binutils-gdb.git gdb
WORKDIR gdb
RUN ./configure --with-expat CFLAGS="-O0 -g -fno-inline" CXXFLAGS="-O0 -g -fno-inline"
RUN make
RUN make install
WORKDIR ..

# QEMU
RUN git clone --branch master --depth 1 --recursive --shallow-submodules --single-branch https://gitlab.com/qemu-project/qemu.git
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

# Set local time.
RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

# Put git settings.
WORKDIR /root
RUN mkdir .ssh
RUN mkdir .github

# Put bash settings.
RUN cat .bash/.bashrc >> ../.bashrc

# Put vim settings.
RUN cat .vim/.vimrc >> ../.vimrc

# Expose VNC port.
ARG vnc_port
EXPOSE $vnc_port


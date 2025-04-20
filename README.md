# HeliOS

This is my original operating system running on x64 architecture.

## A development environment

We assume you are in an environment where you can use [Docker](https://www.docker.com/).
We call this environment the host.
In the host, clone and enter this repository.
And `make environment` to build and enter the development environment.

```
/somewhere/in/the/host $ git clone https://github.com/TaiseiIto/HeliOS.git
/somewhere/in/the/host $ cd HeliOS
/somewhere/in/the/host/HeliOS $ make environment
~/HeliOS #
```
Now you are in the development environment!

## Run HeliOS on QEMU

On the development environment, `make run` to run HeliOS on QEMU.

```
~/HeliOS # make run
```

This command divides terminal screen left and right by tmux.
The left screen shows a log from COM2 of the QEMU.
The right screen is a terminal of the development environment.
Press `Ctrl+t` and press `h` to move from the right screen to the left screen.
Also, press `Ctrl+t` and press `l` to move from the left screen to the right screen.
And Connect from a VNC client on the host to `localhost:5900` according to [RFB protocol](https://datatracker.ietf.org/doc/html/rfc6143) to operate HeliOS.

## Stop HeliOS on QEMU

Move to the right screen and `make stop` to stop HeliOS on QEMU.

```
~/HeliOS # make stop
```

## Debug HeliOS on QEMU by GDB

On the development environment, `make debug` to debug HeliOS on QEMU by GDB.

```
~/HeliOS # make debug
```

This command divides terminal screen left and right by tmux.
The left screen shows a log from COM2 of the QEMU.
The right screen is a GDB terminal attaching QEMU.
Press `Ctrl+t` and press `h` to move from the right screen to the left screen.
Also, press `Ctrl+t` and press `l` to move from the left screen to the right screen.
And Connect from a VNC client on the host to `localhost:5900` according to [RFB protocol](https://datatracker.ietf.org/doc/html/rfc6143) to operate HeliOS.
When you finish debugging HeliOS, move to right screen, `quit` GDB and `make stop`.

```
(gdb) quit
~/HeliOS # make stop
```

## Run HeliOS on a physical machine

`exit` the development environment and `make tree` on the host to generate `helios.mnt`, the HeliOS directory tree.
And copy the generated directory to your storage device.

```
~/HeliOS # exit
/somewhere/in/the/host/HeliOS $ make tree
/somewhere/in/the/host/HeliOS $ cp -r helios.mnt /your/storage/device/
```

Then, eject the storage device and connect it to a physical machine.
Next, configure BIOS settings of the physical machine to boot up from the storage device according to UEFI.
Finally, reboot the physical machine.

## Get development permission (for only developers, not users)

To get development permission, you need to prepare below.

* A SSH key to push to [this repository](https:/github.com/TaiseiIto/HeliOS).
* A `.gnupg` directory to verify your commits.

And `make permission` like below.

```
~/HeliOS # exit
/somewhere/Helios $ make permission SSHKEY=/path/to/ssh/key GPGKEY=/path/to/.gnupg
Your GitHub user name: Someone
Your Github email address: someone@example.com
/somewhere/HeliOS $
```

Now you have development permission!


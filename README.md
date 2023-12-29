# HeliOS

This is my original operating system running on x64 architecture.

## Development environment

HeliOS is developed on a Docker container provided by [.docker](.docker) directory.
You can build and enter a development environment like below.

```
/somewhere $ git clone https://github.com/TaiseiIto/HeliOS.git
/somewhere $ cd HeliOS
/somewhere/HeliOS $ make environment
~/HeliOS #
```
Now you are in the development environment!

## Get development permission (for only developers, not users)

To get development permission, you need to prepare below.

* A SSH key to push to [this repository](https:/github.com/TaiseiIto/HeliOS).
* A `.gnupg` directory to verify your commits.

And `make permission` like below.

```
~/HeliOS # exit
/somewhere/Helios $ make permission GITHUB=/path/to/ssh/key GITGPG=/path/to/.gnupg
Your GitHub user name: Someone
Your Github email address: someone@example.com
Password for someone@example.com: ********
/somewhere/HeliOS $
```

Now you have development permission!


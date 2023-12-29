# Prepare a development environment as a Docker container.
environment:
	make environment -C .docker

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# Usage+ $make permission GITHUB=/path/to/ssh/key GITGPG=/path/to/.gnupg
permission:
	make permission -C .docker GITHUB=$(realpath $(GITHUB)) GITGPG=$(realpath $(GITGPG))


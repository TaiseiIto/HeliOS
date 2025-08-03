#!/bin/sh

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: $ ./permit.sh <domain> <developer> <email> <product> <ssh key> <gpg key>

domain=$1
developer=$2
email=$3
product=$4
ssh_key=$5
gpg_key=$6

current_directory=$(pwd)
work_directory=$(dirname $0)
cd $work_directory

git config --global user.name $developer
git config --global user.email $email
git config --global user.signingkey $(head -n1 $gpg_key/signingkey.txt)
git config --global commit.gpgsign true
git remote set-url origin git@$domain:$developer/$product.git
cat .gitconfig >> $HOME/.gitconfig
cat <<EOF >> $HOME/.ssh/config
Host $domain
	HostName $domain
	IdentityFile $ssh_key
	User git
EOF
chmod 600 $ssh_key
chmod -R 600 $gpg_key

cd $current_directory


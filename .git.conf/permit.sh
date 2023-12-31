#!/bin/sh

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: $ ./permit.sh <domain> <developer> <product> <ssh key> <gpg key>

domain=$1
developer=$2
product=$3
ssh_key=$4
gpg_key=$5

current_directory=$(pwd)
work_directory=$(dirname $0)
cd $work_directory

read -p "Your GitHub user name:" name
read -p "Your GitHub email address:" email

git config --global user.email $email
git config --global user.name $name
git config --global user.signingkey $(head -n1 $gpg_key/signingkey.txt)
git config --global commit.gpgsign true
git remote set-url origin git@$domain:$developer/$product.git
cat .gitconfig >> $HOME/.gitconfig
cat ../.ssh/config  | sed "s|domain|$domain|g" | sed "s|key|$ssh_key|g" >> $HOME/.ssh/config
chmod 600 $ssh_key
chmod -R 600 $gpg_key

cd $current_directory


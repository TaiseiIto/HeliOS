#!/bin/sh

# Get development permission.
# Only developers can execute it.
# Users don:t have to do it.
# This script is called from the Makefile in the same directory.
# Don't execute it directly.
# Usage: $ ./permit.sh <domain> <developer> <product>

domain=$1
developer=$2
product=$3

current_directory=$(pwd)
work_directory=$(dirname $0)
cd $work_directory

read -p "Your GitHub user name:" name
read -p "Your GitHub email address:" email

git config --global user.email $email
git config --global user.name $name
git config --global user.signingkey $(head -n1 $HOME/.gnupg/signingkey.txt)
git config --global commit.gpgsign true
git remote set-url origin git@$domain:$developer/$product.git
cat .gitconfig >> $HOME/.gitconfig
cat ../.ssh/config >> $HOME/.ssh/config
chmod 600 $HOME/.github/key
chmod -R 600 $HOME/.gnupg

cd $current_directory


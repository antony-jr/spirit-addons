#!/usr/bin/env bash

set -x

mkdir -p ~/.spirit/addons

wget -O ~/.spirit/addons/bash-feedback "https://github.com/antony-jr/spirit-addons/releases/download/development/bash-feedback-linux-amd64"
chmod +x ~/.spirit/addons/bash-feedback

wget -O ~/.bashfeedbackrc "https://rawcdn.githack.com/antony-jr/spirit-addons/82c4c6b170738fb3451f015f59de47da58bc4e4b/bash_feedback/bashfeedbackrc.sh"

cp ~/.bashrc ~/.bashrc.backup
echo "source .bashfeedbackrc" >> ~/.bashrc

set +x

if [ -f ~/.spirit/addons/bash-feedback ];
	then
	   echo "[+] Installed Bash Feedback (Dev)"
	else
	   echo "[-] Installation Failed!"
fi

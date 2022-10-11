#!/usr/bin/env bash

set -x

rm -rf ~/.spirit/addons/bash-feedback
rm -rf ~/.bashfeedbackrc
sed -i '/source .bashfeedbackrc/d' ~/.bashrc

set +x

if [ -f ~/.spirit/addons/bash-feedback ];
	then
	   echo "[-] Uninstallation Failed"
	else
	   echo "[+] Uninstalled Bash Feedback. Please Restart your Terminal."
fi

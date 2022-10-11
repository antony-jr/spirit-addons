### spirit trap commands
trap "$HOME/.spirit/addons/bash-feedback error" ERR
PROMPT_COMMAND="if [ 0 -eq \$? ]; then $HOME/.spirit/addons/bash-feedback nonerror; fi"

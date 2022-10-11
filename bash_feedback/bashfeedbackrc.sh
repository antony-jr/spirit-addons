### spirit trap commands
trap "($HOME/.spirit/addons/bash-feedback error > /dev/null 2>&1 &)" ERR
PROMPT_COMMAND="if [ 0 -eq \$? ]; then ($HOME/.spirit/addons/bash-feedback nonerror > /dev/null 2>&1 \&); fi"

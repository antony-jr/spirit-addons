### spirit trap commands
trap "/home/antonyjr/bin/spirit error" ERR
PROMPT_COMMAND="if [ 0 -eq \$? ]; then /home/antonyjr/bin/spirit nonerror; fi"

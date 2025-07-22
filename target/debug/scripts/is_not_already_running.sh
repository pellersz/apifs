#!/bin/bash

if      (( $(pgrep -u $(whoami) apifs | wc -l) == 1 ))
then    exit 0;
else    exit 1;
fi


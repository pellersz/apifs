#!/bin/bash

if      (( $(ps -u $(whoami) | grep apifs | wc -l) == 1 ))
then    exit 0;
else    exit 1;
fi


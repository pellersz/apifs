#!/bin/bash

if      ps -a | grep apifs
then    exit 1;
else    exit 0;


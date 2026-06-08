#!/bin/bash

arg="$1"

if ! [[ "$arg" =~ ^[0-9]+$ ]]; then
    echo "Error: Argument must be an integer."
    exit 1
fi

if (( arg < 40 || arg > 80 )); then
    echo "Error: Value must be between 40 and 80."
    exit 1
fi

sudo bash -c "echo '$arg' > /opt/coolsnap/temp_limit"
#!/bin/bash

# this script is meant to be run from project level
DIFF_FILE=./integration_test/diff.txt

if [[ -s "$DIFF_FILE" ]]; then
    echo "The Integration Test failed"
    echo "process stopped with exit code 1"
    exit 1
fi

exit 0
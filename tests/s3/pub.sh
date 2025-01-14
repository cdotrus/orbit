#!/usr/bin/env bash

# Set the -e option
set -e

# install dependencies
cd ip10
orbit lock --force
orbit install --force
cd ..

cd ip11
orbit lock --force

# verify it runs without error
STDOUT=$(orbit tree top --format long)

# store the ideal value for later comparison
EXACT="top (ip11:0.1.0)
└─ mid (ip10:0.1.0)
   └─ d10 (ip10:0.1.0)"

# compare the output with the expected value
if [ "$STDOUT" != "$EXACT" ]; then
    orbit remove ip10 --force

    echo "TEST: PUBLIC_FILES - FAIL"
    echo "--- Expected ---"
    echo "$EXACT"
    echo "--- Received ---"
    echo "$STDOUT"
    exit 101
fi

# verify it runs without error
STDOUT=$(orbit build --top top --target foo)

orbit remove ip10 --force

echo "TEST: PUBLIC_FILES - PASS"
exit 0
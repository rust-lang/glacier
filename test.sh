#!/bin/bash

set -o pipefail
echo "Testing 14505"
if rustc --version 2>&1 | true; then
  exit 1
fi

for f in src/*
do
  echo "Testing $f:"
  # Compile the code, and if it passes exit with error code
  if rustc "$f" > /dev/null 2>&1; then
    exit 1
  fi
done

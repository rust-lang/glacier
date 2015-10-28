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

echo "Testing 16229"
if bash 16229.sh > /dev/null 2>&1; then
  exit 1
fi

echo "Testing 28586"
if rustc -Z unstable-options --pretty=expanded 28586.rs > /dev/null 2>&1; then
  exit 1
fi

#!/bin/bash

RUSTC=rustc

for f in ices/*
do
  [[ -f $f ]] || continue;
  echo "Testing $f:"
  # Compile the code, and if it exits with a code less than 101, exit with error
  # code.
  #
  # An ICE will cause error code 101. An abort or other signal will cause an
  # error code greater than 101.
  output=$($RUSTC --color=always "$f" 2>&1)
  if (( "$?" < 101 )); then
    echo "$output"
    exit 1
  fi
done

echo "Testing 16229"
if bash 16229.sh > /dev/null 2>&1; then
  exit 1
fi

echo "Finished!"

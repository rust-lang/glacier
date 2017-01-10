#!/bin/bash

echo "Testing 22387"
echo -e '\xD2' | rustc - 2>&1 | grep -q 'internal compiler error' || exit 1

for f in src/*
do
  [[ -f $f ]] || continue;
  echo "Testing $f:"
  # Compile the code, and if it passes exit with error code
  if rustc "$f" > /dev/null 2>&1; then
    exit 1
  fi
done

echo "Testing 21335"
rustc - --out-dir=random_directory_that_does_not_exist/ --emit=llvm-ir <<< 'fn main(){}'

echo "Testing 16229"
if bash 16229.sh > /dev/null 2>&1; then
  exit 1
fi

echo "Finished!"

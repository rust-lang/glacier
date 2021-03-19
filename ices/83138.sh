#!/bin/bash

echo '' | rustc - --crate-name=a --crate-type=lib
echo 'extern crate a;' | rustc - --crate-name=b --crate-type=lib --extern a=liba.rlib
echo 'use b as _;' | rustc - --extern b=libb.rlib --edition=2018

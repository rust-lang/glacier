#!/bin/bash

tmp="$(mktemp -d)"

if [[ ! $tmp || ! -d $tmp ]]
then
    echo "Could not create temporary directory"
    exit 1
fi

cleanup() {
    rm -r "$tmp"
}

trap cleanup EXIT

cd "$tmp"

git clone https://github.com/thiolliere/debug-inconsistence.git && cd debug-inconsistence/runtime

cargo check --target=wasm32-unknown-unknown

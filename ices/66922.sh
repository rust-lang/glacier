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

git clone https://github.com/95th/proc-macro-workshop.git && cd proc-macro-workshop

git checkout ICE

cargo +nightly run

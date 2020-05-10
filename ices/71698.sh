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

cargo new seven-one-six-nine-eight && cd seven-one-six-nine-eight
cargo build
find target -type f -exec gzip {} \;
cargo run

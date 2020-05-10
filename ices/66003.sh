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

git checkout https://github.com/olegnn/proc-macro-ICE.git && cd proc-macro-ICE

RUST_BACKTRACE=full cargo +nightly run

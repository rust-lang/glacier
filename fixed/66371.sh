#!/bin/bash

mkdir temp
cd temp
touch lib.rs
rustc +nightly lib.rs --crate-type=lib -C incremental=incr -C lto=thin
rustc +nightly lib.rs --crate-type=lib -C incremental=incr -C lto=thin

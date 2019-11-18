#!/bin/bash

mkdir temp
cd temp
touch lib.rs
rustc +nightly lib.rs --crate-type=lib -C incremental=incr
rustc +nightly lib.rs --crate-type=lib -C incremental=incr -C save-temps

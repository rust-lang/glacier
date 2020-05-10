#!/bin/bash

cargo new seven-one-six-nine-eight && cd seven-one-six-nine-eight
cargo build
find target -type f -exec gzip {} \;
cargo run

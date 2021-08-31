#!/bin/bash

rustc --edition=2018 -C embed-bitcode=no -C debuginfo=2 --crate-type=lib - << 'EOF'
macro_rules! foo_ { () => {}; }
use foo_ as foo;
EOF

rustc --edition=2018 -C embed-bitcode=no -C debuginfo=2 --extern test_project=$(ls librust_out.*) - << 'EOF'
#[macro_use]
extern crate test_project;

fn main() {
    foo!();
}
EOF

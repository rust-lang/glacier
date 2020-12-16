#!/bin/bash

rustc -Z mir-opt-level=2 -Z instrument-coverage - <<EOF
pub fn main() {
    let c = || {};
    c();
}
EOF

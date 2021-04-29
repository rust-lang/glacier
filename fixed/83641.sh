#!/bin/bash

rustc --crate-type=lib --crate-name=repro - <<'EOF'
#![feature(pub_macro_rules)]
pub macro_rules! fail {
    ($x:expr) => { $x }
}
EOF

rustc -L. - <<'EOF'
extern crate repro;
fn main() {
    repro::fail!(recv);
}
EOF

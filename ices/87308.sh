#!/usr/bin/env bash

rustc - -Zunpretty=everybody_loops << 'EOF'
macro_rules! foo {
    () => { break 'x; } //~ ERROR use of undeclared label `'x`
}

pub fn main() {
    'x: loop { foo!() }
}
EOF

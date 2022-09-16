#!/bin/bash

rustc -Zvalidate-mir - <<'EOF'

#![feature(let_else)]
fn example_let_else(value: Option<String>) {
    let _inner = value else {
        return;
    };
}
fn main() {
    example_let_else(None);
}

EOF


#!/bin/bash

cat > out.rs <<'EOF'

pub use std::*;

pub mod task {}

pub fn main() {
    println!("Hello, world!");
}

EOF

rustdoc out.rs

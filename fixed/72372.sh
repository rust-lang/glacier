#!/bin/bash

cat > 72372.rs <<EOF
fn main() {
    let foo = "1\n2".lines().map(|_| [1]);
    let _ = foo.filter(|_| true);
}

EOF

rustc -Zmir-opt-level=3 72372.rs

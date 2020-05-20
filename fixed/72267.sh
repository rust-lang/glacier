#!/bin/bash

cat > 72267.rs <<EOF
fn main() {
    let _: Box<(?Sized)>;
}
EOF

rustc -Zsave-analysis 72267.rs

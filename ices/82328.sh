#!/bin/bash

rustc -Zunpretty=hir,typed - <<'EOF'
fn main() {}

fn foo(-128..=127: i8) {}
EOF

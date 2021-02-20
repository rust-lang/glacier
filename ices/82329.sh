#!/bin/bash

rustc -Zunpretty=hir,typed - <<'EOF'
pub fn main() {
    if true {
    } else if let a = 1 { //~ WARN irrefutable `if let`
    }
}
EOF

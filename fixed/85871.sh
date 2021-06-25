#!/bin/bash

rustc -Z thir-unsafeck=yes - <<'EOF'
struct Bug {
    inner: [(); match || 1 {
        n => n(),
    }],
}

fn main() {}
EOF

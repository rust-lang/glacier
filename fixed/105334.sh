#!/bin/bash

cat > out.rs <<'EOF'

impl Vec< br##"*.."## > {}

fn main() {}

EOF

rustdoc out.rs

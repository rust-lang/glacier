#!/bin/bash

rustc -Z unpretty=thir-tree - <<'EOF'
pub fn main() {break;}
EOF

#!/bin/bash

rustc --crate-type lib -Zmeta-stats  - <<'EOF'

pub fn a() {}

EOF


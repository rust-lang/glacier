#!/bin/bash

rustc -Z print-type-sizes --crate-type lib - <<'EOF'
use std::str::FromStr;

pub fn foo() {
    f64::from_str("");
}
EOF

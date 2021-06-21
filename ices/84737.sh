#!/bin/bash

rustc --edition=2018 - <<'EOF'
#![allow(incomplete_features)]
#![feature(const_generics)]
#![crate_type="lib"]

async fn test(test: [(); { 0 }]) {
    let _ = &test;
    async {}.await;
}
EOF

#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "abc"
version = "0.0.1"
edition = "2018"
EOF

mkdir -p src

cat > src/test.rs <<'EOF'
#![allow(incomplete_features)]
#![feature(const_generics)]

async fn test(test: [(); { 0 }]) {
    let _ = &test;
    async {}.await;
}
EOF

cargo test

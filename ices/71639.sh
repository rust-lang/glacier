#!/bin/bash

tmp="$(mktemp -d)"

if [[ ! $tmp || ! -d $tmp ]]
then
    echo "Could not create temporary directory"
    exit 1
fi

cleanup() {
    rm -r "$tmp"
}

trap cleanup EXIT

cd "$tmp"

cargo new asm_error && cd asm_error

cat > Cargo.toml <<EOF
[package]
name = "asm_error"
version = "0.1.0"
edition = "2018"

# this causes a bug
[profile.release]
codegen-units = 1
lto = true
EOF

cat > src/lib.rs <<EOF
#![feature(llvm_asm)]
pub struct S;
pub fn f() {
    unsafe { llvm_asm!( "call [rsi + 8*rax]" : : "rbx"(&S) : : "intel" ) };
}
EOF

cat > src/main.rs <<EOF
fn main() { asm_error::f() }
EOF

cargo +nightly build --release

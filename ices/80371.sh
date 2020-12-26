#!/bin/bash

rustc -C llvm-args=--x86-asm-syntax=intel -C opt-level=3 - << 'EOF'
#![crate_type = "lib"]
pub struct Header<'a> {
    pub value: &'a [u8],
}

pub fn test() {
    let headers = [Header{value: &[]}; 128];
}
EOF

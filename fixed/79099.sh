#!/bin/bash

rustc - << 'EOF'
#![feature(impl_trait_in_bindings)]

struct Bug {
    V1: [(); {
        let f: impl core::future::Future<Output = u8> = async { 1 };
        1
    }],
}

fn main() {}
EOF

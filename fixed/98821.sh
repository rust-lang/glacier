#!/bin/bash

rustc -Zmir-opt-level=3 - <<'EOF'

#![feature(unboxed_closures)]

extern "rust-call" fn foo<T>(_: T) {}

fn main() {
    foo(());
    foo((1, 2));
}


EOF


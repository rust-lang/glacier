#!/bin/bash

cat > 72181.rs <<EOF
#![feature(never_type)]
#![allow(unused, invalid_value)]

enum Void {}

fn f(v: Void) -> ! {
    match v {} //~ ERROR entering unreachable code
}

fn main() {
    let v: Void = unsafe {
        std::mem::transmute::<(), Void>(())
    };
    f(v); //~ inside `main`
}

EOF

rustc -Zmir-opt-level=3 --emit=mir 72181.rs

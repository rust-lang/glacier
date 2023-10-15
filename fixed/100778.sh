#!/bin/bash

rustc -Clto -Zsanitizer=cfi - <<'EOF'

#![feature(adt_const_params, generic_const_exprs)]
#![allow(incomplete_features)]

pub type Matrix = [usize; 1];
const EMPTY_MATRIX: Matrix = [0; 1];

pub struct Walk<const REMAINING: Matrix> { }

impl Walk<EMPTY_MATRIX> {
    pub const fn new() -> Self {
        Self {}
    }
}

fn main() {
    let _ = Walk::new();
}

EOF


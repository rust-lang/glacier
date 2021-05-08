#!/bin/bash

rustc --edition 2018 -C incremental=foo --crate-type lib - <<'EOF'
#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

pub struct Ref<'a>(&'a i32);

impl<'a> Ref<'a> {
    pub fn foo<const A: usize>() -> [(); A - 0] {
        Self::foo()
    }
}

EOF


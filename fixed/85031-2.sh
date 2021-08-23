#!/bin/bash

rustc --edition 2018 -C incremental=foo --crate-type lib - <<'EOF'
#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

pub struct Ref<'a>(&'a i32);

impl Ref<'_> {
    pub fn foo<const A: usize>()
    where
        ([(); A - 0], ()): Sized,
    {
        Self::bar::<A>()
    }

    fn bar<const A: usize>()
    where
    ([(); A - 0], ()): Sized,
    {
    }
}

EOF

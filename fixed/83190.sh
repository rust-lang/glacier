#!/bin/bash

rustc --crate-type=rlib - <<'EOF'
#![feature(min_type_alias_impl_trait)]
#![no_std]

pub trait AssociatedImpl {
    type ImplTrait;

    fn f() -> Self::ImplTrait;
}

struct S<T>(T);

trait Associated {
    type A;
}

// ICE
impl<'a, T: Associated<A = &'a ()>> AssociatedImpl for S<T> {
    type ImplTrait = impl core::fmt::Debug;

    fn f() -> Self::ImplTrait {
    //~^ ERROR unexpected concrete region in borrowck: ReEarlyBound(0, 'a)
        ()
    }
}
EOF

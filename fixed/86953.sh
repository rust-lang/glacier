#!/usr/bin/env bash

rustc - --crate-type=lib -C incremental=foo << 'EOF'
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

use std::ops::Add;

pub struct StackBitSet<const N: usize> where [(); 0]: Sized;

impl<const N: usize, const M: usize> Add<&StackBitSet<M>> for StackBitSet<N>
where
    [(); 0+0]: Sized, // changing 0+0 to 0 makes this work
    [(); 0]: Sized,
    [(); 0]: Sized,
{
    type Output = StackBitSet<0>;
    fn add(self, _: &StackBitSet<M>) -> Self::Output {
        loop {}
    }
}

#[test]
fn bitset_create() {
    let _a: StackBitSet<42> = StackBitSet;
}
EOF

#![feature(
    const_assume,
    const_evaluatable_checked,
    const_generics,
    use std::intrinsics::assume;
)]
#![allow(incomplete_features)]

const fn foo(n: usize) -> usize { n }

pub struct Bar<const N: usize>(usize);

impl<const N: usize> Bar<N> where [(); foo(N)]: {
    fn spam(i: usize) { }
}

fn main() {}

#![allow(incomplete_features)]
#![feature(const_trait_impl)]
#![feature(const_closures)]
#![feature(const_refs_to_cell)]
#![feature(type_alias_impl_trait)]

pub type Diff = impl ~const std::marker::Destruct + ~const Fn(usize) -> usize;

pub const fn lift(n: usize) -> Diff {
    const move |m: usize| m + n
}

pub const fn reify(n: Diff) -> usize {
    n(0)
}

pub const fn add(n: Diff, m: Diff) -> Diff {
    const move |x: usize| m(n(x))
}

fn main() {}

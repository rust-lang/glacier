#![feature(const_generics)]

pub type Array<T, const N: usize> = [T; N];

pub fn foo<const N: usize>() -> Array<N, ()> {
    unimplemented!()
}

fn main() {}

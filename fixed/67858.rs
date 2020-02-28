#![feature(const_generics)]
#![allow(incomplete_features)]

pub trait Trait {
    type Associated;
}

pub struct Foo<T, A = <T as Trait>::Associated, const N: u8>(T, A);

impl<T: Trait, const N: u8> Foo<T, N> {
}

fn main() {}

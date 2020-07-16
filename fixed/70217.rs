#![feature(const_generics)]
#![allow(incomplete_features)]

pub struct Struct<const N: usize>;

impl<const N: usize> Struct<N> {
    fn method<const M: usize>(&self) {}
}

pub fn test<const N: usize, const M: usize>(x: Struct<N>) {
    Struct::<N>::method::<M>(&x);
    x.method::<N>();
}

fn main() {}

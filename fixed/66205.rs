#![feature(const_generics)]

fn fact<const N: usize>() {
    fact::<{ N - 1 }>();
}

fn main() {}

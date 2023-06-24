#![feature(inherent_associated_types)]
#![allow(incomplete_features, dead_code, unused_variables)]

struct Foo;

struct Bar<const X: usize>([(); X]);

impl<const X: usize> Bar<X> {
    pub fn new() -> Self {
        Self([(); X])
    }
}

impl Foo {
    type Bar<const X: usize> = Bar<X>;
}

fn main() {
    let a = Foo::Bar::<10usize>::new();
}
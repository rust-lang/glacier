#![feature(never_type)]
#![feature(specialization)]

use std::iter::{self, Empty};

trait Trait {
    type Out: Iterator<Item = u32>;

    fn f(&self) -> Option<Self::Out>;
}

impl<T> Trait for T {
    default type Out = !;

    default fn f(&self) -> Option<Self::Out> {
        None
    }
}

struct X;

impl Trait for X {
    type Out = Empty<u32>;

    fn f(&self) -> Option<Self::Out> {
        Some(iter::empty())
    }
}

fn main() {
    if let Some(iter) = 1.f() { // replacing 1 by X makes the code compiles and runs fine
        println!("Some");
        for x in iter {
            println!("x = {}", x);
        }
    }
}

#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![allow(incomplete_features)]

trait Trait1 {
    fn foo();
}

trait Trait2 {
    type Associated: Trait1 = Self;
}

impl Trait2 for () {}

fn call_foo<T: Trait2>() {
    T::Associated::foo()
}

fn main() {
    call_foo::<()>()
}

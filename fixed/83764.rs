#![allow(dead_code)]

pub fn main() {
    // Instantiating Foo with a concrete type will not fail
    let _ = Foo::<dyn FooTrait>::new();
}

pub struct Foo<T: FooTrait + ?Sized> {
    base: FooBase,
    value: T,
}

impl<T: FooTrait + ?Sized> Foo<T> {
    pub fn new() -> Box<Foo<T>> {
        todo!()
    }
}

pub trait FooTrait {}

pub struct FooBase {
    cls: Bar,
}

// Bar *must* be a fieldless enum
pub enum Bar {}

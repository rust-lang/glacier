#![feature(const_generics)]

struct Foo;

impl Foo {
    fn foo<const N: usize>(&self) {}
}

fn main() {
    Foo.foo::<0usize>();
}

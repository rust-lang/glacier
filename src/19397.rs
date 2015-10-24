#![feature(unboxed_closures)]

trait Foo {
    extern "rust-call" fn foo();
}

impl Foo for () {
    extern "rust-call" fn foo() {}
}

fn main() {}

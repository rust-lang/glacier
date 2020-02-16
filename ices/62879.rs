#![feature(const_generics)]

fn foo<const N: usize, const A: [u8; N]>() {}

fn bar() {
    foo::<1, {[1]}>();
}

fn main() {}

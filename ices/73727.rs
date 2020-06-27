#![feature(const_generics)]
#![allow(incomplete_features)]

fn a<const X: &'static [u32]>() {}

fn main() {
    a::<{ &[] }>();
}

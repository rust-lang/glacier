#![allow(incomplete_features)]
#![feature(impl_trait_in_bindings)]

fn main() {
    let ref _x: impl Sized = 5;
}

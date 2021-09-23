#![feature(adt_const_params)]
#![allow(incomplete_features)]

fn a<const X: &'static [u32]>() {}

fn main() {
    a::<{ &[] }>();
}

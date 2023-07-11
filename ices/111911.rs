#![feature(adt_const_params)]

fn foo<const N: &'static u8>() -> impl Sized {}

fn main() {}

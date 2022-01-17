#![feature(generic_const_exprs)]

struct Foo<const A: [(); 0 + 0]>
    where [(); 0 + 0]: Sized;

pub fn main() {}

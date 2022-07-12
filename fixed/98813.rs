#![feature(adt_const_params)]

const ZERO: f64 = 0.0;

pub struct Foo<const X: f64> {}

fn main() {
    let _ = Foo::<0.0> {}; // fails
    let _ = Foo::<ZERO> {}; // passes
}

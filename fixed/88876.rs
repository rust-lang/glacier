#![allow(incomplete_features)]
#![feature(adt_const_params)]

struct FooConst<const ARRAY: &'static [&'static str]> {}

const FOO_ARR: &[&'static str; 2] = &["Hello", "Friend"];

fn main() {
    // Commenting the following line avoids the panic.
    let foo = FooConst::<FOO_ARR> {};
}

#![allow(incomplete_features)]
#![feature(adt_const_params)]

struct LifetimeGeneric<'a>(&'a ());

struct UwU<'b, const T: LifetimeGeneric>(&'b ());

fn main() {}

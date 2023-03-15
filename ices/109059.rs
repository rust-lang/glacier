#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![allow(dead_code)]

#[derive(PartialEq, Eq)]
struct S;

trait T<const C: &'static S> {}

fn foo<const C: &'static S>(t: impl T<C>) -> impl T<C> {
    t
}

fn main() {}

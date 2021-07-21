#![feature(impl_trait_in_bindings)]

trait Trait {}
impl Trait for () {}

fn foo<'a: 'a>() {
    let _x: impl Trait = ();
}

fn main () {}

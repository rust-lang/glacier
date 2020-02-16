#![feature(impl_trait_in_bindings)]

struct A<'a>(&'a ());

trait Trait<T> {
}

impl<T> Trait<T> for () {
}

fn main() {
    let x: impl Trait<A> = ();
}

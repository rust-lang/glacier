#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]

trait Trait {
    type Type;

    fn method(&self) -> impl Trait<Type = impl Trait<Type = impl Sized + '_> + '_>;
}

fn main() {}

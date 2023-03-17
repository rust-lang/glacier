#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

trait Iterator {
    type Item;
}

trait IntoIterator {
    fn into_iterator(&self) -> impl Iterator<Item: > + '_;
}

struct IntoIterFn<F, I> {
    f: F,
    _marker: core::marker::PhantomData<I>,
}

impl<F, I> IntoIterator for IntoIterFn<F, I>
where
    F: Fn() -> I,
    I: Iterator,
{
    fn into_iterator(&self) -> impl Iterator<Item: '_> {
        (self.f)()
    }
}

fn main() {}

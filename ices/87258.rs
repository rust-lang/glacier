#![feature(min_type_alias_impl_trait)]
#![feature(generic_associated_types)]
use std::future::Future;

pub trait Trait1
{
    type Error: Clone;
    type Assoc2<'a>
    where
        Self: 'a;
}

pub trait Trait2: Trait1 {
    type FooFuture<'a, 'b>: Future<Output = Result<(), Self::Error>>;
    fn foo<'a: 'b, 'b>(assoc2: &'b mut Self::Assoc2<'a>) -> Self::FooFuture<'a, 'b>
    where
        Self: 'a;
}

impl<'c, S : Trait1> Trait1 for &'c mut S {
    type Error = S::Error;

    type Assoc2<'a> where 'c: 'a = S::Assoc2<'a>;
}

impl<'c, S : Trait2> Trait2 for &'c mut S {
    type FooFuture<'a, 'b> = impl Future<Output = Result<(), Self::Error>>;
    fn foo<'a: 'b, 'b>(assoc2: &'b mut Self::Assoc2<'a>) -> Self::FooFuture<'a, 'b>
    where
        Self: 'a
    {
        async move {
            unimplemented!();
        }
    }
}

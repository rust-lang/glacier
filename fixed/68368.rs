#![feature(type_alias_impl_trait)]

trait Trait<T> {}

type Alias<'a, U> = impl Trait<U>;

fn f<'a>() -> Alias<'a, ()> {}

fn main() {}

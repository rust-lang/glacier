#![feature(return_position_impl_trait_in_trait)]

struct Wrapper<G: Send>(T);

trait Foo {
    fn bar() -> Wrapper<impl Sized>;
}

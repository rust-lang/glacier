#![feature(const_generics)]
trait Foo {}

impl<const N: usize> Foo for [(); N]
where
    Self:FooImpl<{N==0}>
{}

trait FooImpl<const IS_ZERO: bool>{}

fn main() {}

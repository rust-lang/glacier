#![feature(trait_alias)]

struct Bar;
trait Foo {}
impl Foo for Bar {}

trait Baz = Foo where Bar: Foo;

fn new() -> impl Baz {
    Bar
}

fn main() {}

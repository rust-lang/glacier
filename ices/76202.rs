#![feature(type_alias_impl_trait)]

trait Dummy {}
impl Dummy for () {}

type F = impl Dummy;
fn f() -> F {}

trait Test {
    fn test(self);
}

impl Test for F {
    fn test(self) {}
}

fn main() {
    let x: F = f();
    x.test();
}

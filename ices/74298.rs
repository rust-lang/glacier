#![feature(type_alias_impl_trait)]

type X<T> = impl Sized;

fn f<T>() -> X<T> {}

trait Y {
    fn g(&self) {}
}

impl Y for X<()> {}
impl Y for X<i32> {}

fn main() {
    f::<()>().g();
}

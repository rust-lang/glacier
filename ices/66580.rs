#![feature(type_alias_impl_trait)]

type Closure = impl FnOnce();

fn closure() -> Closure {
    || {}
}

struct Wrap<T> { f: T }

impl Wrap<Closure> {}

impl<T> Wrap<T> {}

fn main() {}

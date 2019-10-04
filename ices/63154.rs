#![allow(dead_code)]

trait HasAssocType {
    type Inner;
}

impl HasAssocType for () {
    type Inner = ();
}

trait Tr<I, T>: Fn(I) -> Option<T> {}
impl<I, T, Q: Fn(I) -> Option<T>> Tr<I, T> for Q {}

fn f<T: HasAssocType>() -> impl Tr<T, T::Inner> {
    |_| None
}

fn g<T, Y>(f: impl Tr<T, Y>) -> impl Tr<T, Y> {
    f
}

fn h() {
    g(f())(());
}

fn main() {}

#![feature(type_alias_impl_trait)]

type X<T> = impl Clone;

fn f<T: Clone>(t: T) -> X<T> {
    t
}

fn g<T>(o : Option<X<T>>) -> Option<X<T>> {
    o.clone()
}

fn main() {
    g(None::<X<&mut ()>>);
}

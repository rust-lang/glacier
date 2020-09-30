#![feature(type_alias_impl_trait)]

type Pointer<T> = impl std::ops::Deref<Target=T>;

fn test() -> Pointer<_> {
    Box::new(1)
}

fn main() {
    test();
}

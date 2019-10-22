#![feature(type_alias_impl_trait)]

type Fut = impl std::future::Future;

fn take(_: fn() -> Fut) {}

fn main() {
    take(|| {});
    take(|| {});
}

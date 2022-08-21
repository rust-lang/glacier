#![feature(type_alias_impl_trait)]
use std::future::Future;

fn main() {
    type SomeFuture<'t> = impl 't + Future<Output = ()>;
    type SomeClosure = impl for<'t> FnOnce(&'t str) -> SomeFuture<'t>;
    fn coerce_closure(f: SomeClosure) {}
    coerce_closure(|x: &str| async move {});
}

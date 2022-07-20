#![feature(type_alias_impl_trait)]
#![crate_type = "lib"]

use std::future::Future;

trait Bar {
    fn bar(&self);
}

type FooFuture<B> = impl Future<Output = ()>;

fn foo<B: Bar>(bar: B) -> FooFuture<B> {
    async move { bar.bar() }
}

pub fn mainish(ctx: &mut std::task::Context) {
    let boom: FooFuture<u32> = unsafe { core::mem::zeroed() };
    Box::pin(boom).as_mut().poll(ctx);
}

#![feature(type_alias_impl_trait)]

use std::future::Future;

type FutNothing<'a> = impl 'a + Future<Output = ()>;

async fn operation(x: &mut ()) -> () {
    ()
}

async fn indirect() {
    call(operation).await
}

async fn call<F>(mut f: F)
where
    for<'any> F: FnMut(&'any mut ()) -> FutNothing<'any>,
{
    f(&mut ()).await
}

fn main() {}

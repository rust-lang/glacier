#![crate_type = "rlib"]

use core::future::Future;

async fn handle<F>(slf: &F)
where
    F: Fn(&()) -> &mut (dyn Future<Output = ()> + Unpin),
{
    (slf)(&()).await;
}

#![feature(unsized_fn_params, unsized_locals)]
#![crate_type = "lib"]

use std::future::Future;

async fn bug<T>(mut f: dyn Future<Output = T> + Unpin) -> T {
    (&mut f).await
}

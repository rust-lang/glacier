#![crate_type = "lib"]
#![feature(async_fn_in_trait)]
trait T {
    async fn foo();
}

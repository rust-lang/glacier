#![feature(async_await)]
#![feature(unsized_locals)]

async fn f() {}

async fn g(x: Box<dyn core::fmt::Display>) {
    let _x = *x;
    f().await;
}

fn main() {
    let _a = g(Box::new(5));
}

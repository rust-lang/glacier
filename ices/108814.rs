#![feature(non_lifetime_binders)]

fn take(_: impl for<T> FnOnce(T) -> T) {}

fn main() {
    take(|x| x)
}


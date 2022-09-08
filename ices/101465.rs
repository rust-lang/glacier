#![feature(trait_alias)]

struct B;

struct C;

trait Tr2<S> = Into<S>;

fn foo2<T: Tr2<()>>() {}

fn foo() -> impl Sized {
    let x = foo2::<_>();

    match true {
        true => B,
        false => C,
    }
}

pub fn main() {}

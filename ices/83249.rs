#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait Foo {
    const N: usize;
}

impl Foo for u8 {
    const N: usize = 1;
}

fn foo<T: Foo>(_: [u8; T::N]) -> T {
    todo!()
}

pub fn bar() {
    // This equivalent line does not ICE
    //foo::<u8>([0; 1]);
    let _: u8 = foo([0; 1]);
}

fn main() {}

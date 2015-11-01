#![feature(associated_consts, const_fn)]
struct A(u32);

impl A {
    const fn new() -> A { A(0) }
    const TEST: A = A::new();
}

fn main() {
    match A(1) {
        A::TEST => (),
        _ => ()
    }
}

#![feature(unboxed_closures)]

struct A;
impl A {
    extern "rust-call" fn b() {}
}

fn main () {
    A::b();
}

#![feature(unboxed_closures,core)]

pub struct Foo;

impl<A> FnOnce<(A,)> for Foo {
    type Output = ();
    extern "rust-call" fn call_once(self, (_,): (A,)) {
    }
}

pub fn main() {
    println!("{:?}", Foo("bar"));
}

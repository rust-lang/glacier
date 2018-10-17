#![feature(fn_traits, unboxed_closures)]

struct A;
impl A {
    extern "rust-call" fn foo() {}
}

trait T {
    extern "rust-call" fn foo(i: i32) {}
}

struct B;

impl T for B {}

struct C;

impl FnOnce<i64> for C {
    type Output = ();
    extern "rust-call" fn call_once(mut self, _: i64) {}
}

fn main () {
    A::foo();
    <B as T>::foo(10);
}

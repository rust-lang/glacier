#![feature(const_generics)]

struct T;
impl T {
    fn new_s<const N: u8>() -> S<N> { S }
}

struct S<const N: u8>;
impl<const N: u8> S<N> {
    fn method(&self) {}
}

fn main() {
    T::new_s::<1u8>().method();
}

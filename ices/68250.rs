#![allow(unused)]
#![feature(const_generics)]

trait Child {
    const N: usize;
}

struct C1;
impl Child for C1 {
    const N: usize = 2;
}

struct Main<C: Child> {
    c: C
}

impl<C: Child> Main<C> {
    fn test() -> [u8; C::N] {
        [0; C::N]
    }
}

// Doesn't crash without `fn main()` or `#![crate_type = "lib"]`
fn main() {}

#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

pub struct Foobar<T, const N: usize> {
    t: T,
}

pub trait Footrait {
    const N: usize;
}

impl<T> Footrait for T {
    const N: usize = 0;
}

pub fn new_foo<T>(t: T) -> Foobar<T, { <T as Footrait>::N }>
where
    T: Footrait,
{
    Foobar { t }
}

fn main() {
    let foo = new_foo(0);
}

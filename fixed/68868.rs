#![feature(const_generics)]

pub trait Foo<T, const X: usize> {
    fn foo(&self);
}

pub trait Bar<T, const X: usize> {
    fn bar(&self);
}

impl<const X: usize> Bar<u32, X> for u32
where
    u8: Foo<X, u16>,
    //u8: Foo<u32, X>,
    //u8: Foo<{ X }, u16>,
{
    fn bar(&self) {
        0u8.foo();
    }
}

fn main() {}

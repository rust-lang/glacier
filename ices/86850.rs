#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![allow(incomplete_features)]
use core::marker::PhantomData;

pub trait Foo {
    const SIZE: usize;
}

pub struct Bar<T>(PhantomData<T>);
impl<T: Foo> Bar<T> {
    pub fn new(array: [(); T::SIZE]) -> Self {
        Self(PhantomData)
    }
    pub fn bar(self) -> Bar<T>
    where
        [(); T::SIZE]: ,
    {
        Bar::new([(); T::SIZE])
    }
}

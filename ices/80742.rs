#![feature(const_evaluatable_checked)]
#![feature(const_generics)]
#![allow(incomplete_features)]

use core::fmt::Debug;
use core::marker::PhantomData;

struct Inline<T>
where
    [u8; ::core::mem::size_of::<T>() + 1]: ,
{
    _phantom: PhantomData<T>,
    buf: [u8; ::core::mem::size_of::<T>() + 1],
}

impl<T> Inline<T>
where
    [u8; ::core::mem::size_of::<T>() + 1]: ,
{
    pub fn new(val: T) -> Inline<T> {
        todo!()
    }
}

fn main() {
    // let dst = Inline::<usize>::new(0); OK
    let dst = Inline::<dyn Debug>::new(0); // BANG!
}

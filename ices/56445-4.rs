#![crate_type = "lib"]

use std::marker::PhantomData;

struct S<'a>
{
    m1: PhantomData<&'a u8>,
    m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    const fn size() -> usize { 1 }

    fn new() -> Self
    {
        Self
        {
            m1: PhantomData,
            m2: [0; Self::size()],
        }
    }
}

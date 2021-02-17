#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
pub struct SimpleStruct<const N: usize>([u8; N]);

impl<const N: usize> SimpleStruct<N> {
    pub fn new() -> Self {
        loop {}
    }
}

pub trait TraitA {
    const SIZE: usize;
    fn zero()
    where
        [(); Self::SIZE]: ,
    {
        let _ = SimpleStruct::<{ Self::SIZE }>::new();
    }
}

fn main() {}

#![feature(const_generics, const_evaluatable_checked)]

pub trait Ice {
    type Array;

    fn into_bytes(self) -> Self::Array;
}

impl<T: Ice, const N: usize> Ice for [T; N]
where
    [(); std::mem::size_of::<T::Array>() * N]: ,
{
    type Array = [(); std::mem::size_of::<T::Array>() * N];

    fn into_bytes(self) -> Self::Array {}
}

fn main() {}

#![feature(const_generics)]

pub struct Tuple;

pub trait ConstInput<const I: usize> {
    type Input;
}

impl Tuple {
    fn const_index<const I: usize>(_: <Self as ConstInput<I>>::Input)
    where
        Self: ConstInput<I>
    {}
}

fn main() {}

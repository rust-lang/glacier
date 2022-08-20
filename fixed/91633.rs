use std::ops::Index;

pub struct Handle(u64);

impl<T> Index<Handle> for [T] where Self: Index<usize> {
    type Output = <Self as Index<usize>>::Output;

    fn index(&self, index: Handle) -> &Self::Output {
        &self[index.0 as usize]
    }
}

pub fn main() {}

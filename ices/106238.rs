#![feature(trait_alias)]
use core::ops::Add;

pub trait DoSome<T> {}

// Trait alias causes compiler panic
pub trait Cell<T: Add<T, Output=T>> = DoSome<T>;

struct _Container<T> {
    pub cells: dyn Cell<T>,
}


fn main() {}


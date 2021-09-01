#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

pub trait IsTrue<const T: bool> {}
impl IsTrue<true> for () {}

pub trait IsZST {}

impl<T> IsZST for T
where
    (): IsTrue<{ std::mem::size_of::<T>() == 0 }>
{}

fn func() -> impl IsZST {
    || {}
}

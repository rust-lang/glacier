#![feature(ptr_metadata)]

pub struct S;

impl S
where
    ():,
{
    fn f<T: ?Sized>(_: <T as core::ptr::Pointee>::Metadata) {}
}

pub fn main() {}

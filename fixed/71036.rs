#![feature(unsize, dispatch_from_dyn)]

use std::marker::Unsize;
use std::ops::DispatchFromDyn;

struct Inner<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Inner<'a, U>> for Inner<'a, T> {}

impl<'a, T: ?Sized> Inner<'a, T> {
    fn new(value: &'a T) -> Inner<'a, T> {
        Inner { value }
    }
}

pub struct Local<'a, T: ?Sized> {
    inner: &'a Inner<'a, T>,
}

impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<Local<'a, U>> for Local<'a, T> {}

impl<'a, T: ?Sized> Local<'a, T> {
    fn new(inner: &'a Inner<'a, T>) -> Local<'a, T> {
        Local { inner }
    }
}

fn main() {}

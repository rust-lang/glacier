#![feature(inline_const)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::mem;

pub union AsBytes<T> {
    pub value: mem::ManuallyDrop<T>,
    pub as_bytes: [u8; const { mem::size_of::<T>() }],
}

fn main() {}

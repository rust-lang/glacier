#![feature(allocator_api)]

use std::alloc::{Allocator, Global, Layout};

fn main() {
    let layout: Layout = None.unwrap();
    let ptr: *mut u8 = Global.allocate(layout).unwrap().as_ptr() as _;
    let box_ = unsafe { Box::from_raw_in(ptr, &Global) };
}

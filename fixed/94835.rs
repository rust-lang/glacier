#![feature(allocator_api)]
use std::alloc::Global;

fn main() {
    // f(()) // no ICE
    f(32); // ICE
}

pub fn f<T>(val: T) {
    *Box::new_in(val, &Global);
}

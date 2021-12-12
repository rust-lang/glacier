#![feature(extern_types)]

extern "C" {
    type Content;
}

pub struct List {
    len: usize,
    _opaque: Content,
}

pub const Z: usize = {
    let a = [0usize; 4];
    unsafe { &*(&a as *const _ as *const List) }.len
};

pub fn main() {}

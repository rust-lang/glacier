#![feature(extern_types)]

extern "C" {
    type Content;
}

pub struct List {
    len: usize,
    _opaque: Content,
}

pub fn main() {
    let _len = match Option::<&List>::None {
        Some(a) => a.len,
        None => 0,
    };
}

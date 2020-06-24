#![allow(incomplete_features)]
#![feature(const_compare_raw_pointers)]
#![feature(const_generics)]

use std::ffi::{CStr, CString};

fn main() {
    let baguette = CString::new("baguette").unwrap();
    let ptr = baguette.as_ptr();
    println!("{}", unsafe {
        unsafely_do_the_thing::<safely_do_the_thing>(ptr)
    });
}

unsafe fn unsafely_do_the_thing<const F: fn(&CStr) -> usize>(ptr: *const i8) -> usize {
    F(CStr::from_ptr(ptr))
}

fn safely_do_the_thing(s: &CStr) -> usize {
    s.to_bytes().len()
}

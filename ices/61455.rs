#![feature(const_generics)]
#![feature(const_compare_raw_pointers)]

use std::mem::transmute;

struct Bug<const N: fn(usize)>;

fn main() {
    let x = Bug::<{
        unsafe { transmute(|x: u8| {}) }
        
    }>;
}

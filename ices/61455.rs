#![feature(const_generics)]

use std::mem::transmute;

struct Bug<const N: fn(usize)>;

fn main() {
    let x = Bug::<{
        unsafe { transmute(|x: u8| {}) }
        
    }>;
}

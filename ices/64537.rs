#![feature(const_generics)]

struct S;

impl S {
    pub fn x<const I: usize>() {}
    pub fn y() {
        Self::x::<{3usize}>();
    }
}

fn main() {}

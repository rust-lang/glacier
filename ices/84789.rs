#![feature(trait_alias)]

pub trait MyFn = Fn(&mut Self);

// 1.
pub type F = dyn MyFn;

// 2.
pub fn f(_f: &dyn MyFn) {
    
}

fn main() {}

#![allow(incomplete_features)]
#![feature(generic_const_exprs, non_lifetime_binders)]

pub fn foo()
where
    for<const N: usize = { const fn bar() {} bar(); 1 }> ():, 
{}

fn main() {}

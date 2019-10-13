#![feature(impl_trait_in_bindings)]

use std::ops::Add;

fn main() {
    let i: i32 = 0;
    let j: &impl Add = &i;
}

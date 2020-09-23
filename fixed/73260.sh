#!/bin/bash

rustc -Z save-analysis - << EOF
#![feature(const_generics)]
#![allow(incomplete_features)]
struct Arr<const N: usize>
where Assert::<{N < usize::max_value() / 2}>: IsTrue,
{
}

enum Assert<const CHECK: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

fn main() {
    let x: Arr<{usize::max_value()}> = Arr {};
}
EOF

#!/bin/bash

rustc --crate-type=lib --crate-name=ordes - <<'EOF'
#![feature(const_generics, const_evaluatable_checked, array_map)]

use std::array::IntoIter;

pub struct ConstCheck<const CHECK: bool>;

pub trait True {}
impl True for ConstCheck<true> {}

pub trait OrdesDec {
    type Newlen;
    type Output;

    fn pop(self) -> (Self::Newlen, Self::Output);
}

impl<T, const N: usize> OrdesDec for [T; N]
where
    ConstCheck<{N > 1}>: True,
    [T; N - 1]: Sized,
{
    type Newlen = [T; N - 1];
    type Output = T;

    fn pop(self) -> (Self::Newlen, Self::Output) {
        let mut iter = IntoIter::new(self);
        let end = iter.next_back().unwrap();
        let new = [(); N - 1].map(move |()| iter.next().unwrap());
        (new, end)
    }
}
EOF

rustc -L. - <<'EOF'
extern crate ordes;
use ordes::OrdesDec;

fn main() {
    let foo = [0u8, 1, 2, 3, 4];
    let (foo, pop) = foo.pop();
}
EOF

#!/bin/bash

rustc -Z mir-opt-level=4 - << EOF
#![feature(const_generics, box_syntax)]
#![allow(incomplete_features)]

fn main() {
    fn foo<const N: usize>() {
        box [0; N];
    }
    foo::<1>();
}

EOF

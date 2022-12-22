#!/bin/bash

rustc -C opt-level=3 - << EOF
#![crate_type = "lib"]

pub struct A;
pub struct B;

impl Drop for A {
    fn drop(&mut self) {}
}
impl Drop for B {
    fn drop(&mut self) {}
}

#[inline(always)]
pub fn no_unwind() {}

pub fn weird_temporary(a: A, b: B, nothing: ((), (), ()), x: bool) -> ((), (), ()) {
    'scope: {
        (
            {
                let _z = b;
                if x {
                    break 'scope nothing;
                }
            },
            match { a } {
                _ => (),
            },
            no_unwind(),
        )
    }
}

EOF

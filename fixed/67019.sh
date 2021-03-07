#!/bin/bash

rustc +nightly --edition 2018 -Z mir-opt-level=3 - << EOF
fn test(this: ((u8, u8),)) {
    assert!((this.0).1 == 0);
}
fn main() {
    test(((1, 2),));
}

EOF

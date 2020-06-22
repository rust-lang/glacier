#!/bin/bash

rustc -Z mir-opt-level=3 - << EOF

fn main() {
    let i = (0..usize::max_value()).chain(0..10).skip(usize::max_value());
    assert_eq!(i.count(), 10);
}

EOF

#!/bin/bash

mkdir temp 
cd temp
cat > temp.rs << EOF

fn test(this: ((u8, u8),)) {
    assert!((this.0).1 == 0);
}
fn main() {
    test(((1, 2),));
}

EOF

rustc --edition 2018 temp.rs -Z mir-opt-level=2

#!/bin/bash

rustc -g - << EOF

pub fn main() {
let _a = [(); 1 << 63];
}

EOF

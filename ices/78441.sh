#!/bin/bash

rustc -Z save-analysis - << EOF
fn main() {
    [(); { for _ in 0usize.. {}; 0}];
}

EOF

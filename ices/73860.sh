#!/bin/bash

rustc -Zvalidate-mir - << EOF
fn main() {
    for _ in &[0] {}
}

EOF

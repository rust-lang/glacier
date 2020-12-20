#!/bin/bash

rustc -Zchalk - << 'EOF'
const FOO: &&&u32 = &&&42;

fn main() {
    match unimplemented!() {
        &&&42 => {},
        FOO => {},
        _ => {},
    }
}
EOF

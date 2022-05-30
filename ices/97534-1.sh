#!/bin/sh

rustc --edition=2021 - << EOF

macro_rules! m {
    () => {
        macro_rules! foo {
            () => {
                
            }
        }
        use foo as bar;
    }
}

m!{}

use bar as baz;

baz!{}

pub fn main() {}

EOF

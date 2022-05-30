#!/bin/sh

rustc --edition=2021 - << EOF

macro_rules! foo {
    () => {};
}

macro_rules! m {
    () => {
        use foo as bar;
    };
}

m! {}

use bar as baz;

baz! {}

pub fn main() {}

EOF

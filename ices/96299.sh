#!/bin/sh

rustc --edition=2021 - << EOF

enum E {
    S(u8),
}

fn main() {
    let x = E::S(0);
    || {
        let E::S(_x) = x;
    };
}

EOF

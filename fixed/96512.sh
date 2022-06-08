#!/bin/sh

rustc --edition=2021 - << EOF

struct Struct;
enum Enum { Variant(Struct) }
fn main() {
    let _enum = Enum::Variant(Struct);
    || {
        let Enum::Variant(_value) = _enum;
    };
}

EOF

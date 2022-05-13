#!/usr/bin/env bash

rustc --crate-type lib -Zmir-opt-level=3 - 2>&1 << EOF

pub fn foo<'a, T>(s: &'a mut ()) where &'a mut (): Clone {
    <&mut () as Clone>::clone(&s);
}

fn main() {}

EOF

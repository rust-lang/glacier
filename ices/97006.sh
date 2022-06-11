#!/bin/bash

cat > out.rs <<'EOF'

#![allow(unused)]

macro_rules! m {
    ($attr_path: path) => {
        #[$attr_path]
        fn f() {}
    }
}

m!(inline<u8>); //~ ERROR: unexpected generic arguments in path

fn main() {}

EOF

rustc -Zunpretty=hir out.rs

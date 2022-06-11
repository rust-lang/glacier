#!/bin/bash

cat > out.rs <<'EOF'

pub mod m {
    pub struct S;
}

pub trait F {
    fn f() -> m::S;
}

impl<T> F for T {
    fn f() -> m::S {
        m::S
    }
}

EOF

rustdoc --edition=2021 -Z unstable-options --output-format json out.rs

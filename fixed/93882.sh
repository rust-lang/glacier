#!/bin/bash

cat > out.rs <<'EOF'
#![feature(generic_const_exprs)]
pub trait Const {
    const N: usize;
}
pub trait Arr<T>: Const {
    type Arr;
}
impl<T, C: Const> Arr<T> for C
where
    [T; C::N]:,
{
    type Arr = [T; C::N];
}
EOF

rustdoc --edition=2021 out.rs

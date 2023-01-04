#!/bin/bash

cat > out.rs << EOF
use std::ops::Index;

fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
    let _ = s;
}

trait SVec: Index<<Self as SVec>::Item, Output = <Index<<Self as SVec>::Item, Output = <Self as SVec>::Item> as SVec>::Item> {
    type Item<'a, T>;

    fn len(&self) -> <Self as SVec>::Item;
}

fn main() {}

EOF

rustdoc -Znormalize-docs out.rs

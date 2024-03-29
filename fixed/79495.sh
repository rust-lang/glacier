#!/bin/bash

cat > out.rs <<'EOF'
#![feature(arbitrary_enum_discriminant, generic_const_exprs, core_intrinsics)]

extern crate core;
use core::intrinsics::discriminant_value;

#[repr(usize)]
enum MyWeirdOption<T> {
    None = 0,
    Some(T) = std::mem::size_of::<T>(),
    //~^ ERROR constant expression depends on a generic parameter
}

fn main() {
    assert_eq!(discriminant_value(&MyWeirdOption::<u8>::None), 0);
    assert_eq!(discriminant_value(&MyWeirdOption::Some(0u8)), 1);
}

EOF

rustdoc --edition=2021 out.rs

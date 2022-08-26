#!/bin/bash

rustc -Copt-level=2 --crate-type lib  - <<'EOF'

pub trait Trait {
    type Associated;
}
impl<T> Trait for T {
    type Associated = T;
}

pub struct Struct<T>(<T as Trait>::Associated);

pub fn foo<T>() -> Struct<T>
where
    T: Trait,
{
    bar()
}

#[inline]
fn bar<T>() -> Struct<T> {
    Struct(baz())
}

fn baz<T>() -> T {
    unimplemented!()
}

EOF


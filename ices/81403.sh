#!/bin/bash

rustc --emit=mir -Zmir-opt-level=3 - <<'EOF'
// build-pass
pub trait Foo<'a> {
    type Bar;
    fn foo(&'a self) -> Self::Bar;
}

impl<'a, 'b, T: 'a> Foo<'a> for &'b T {
    type Bar = &'a T;
    fn foo(&'a self) -> &'a T {
        self
    }
}

pub fn uncallable<T, F>(x: T, f: F)
    where T: for<'a> Foo<'a>,
          F: for<'a> Fn(<T as Foo<'a>>::Bar)
{
    f(x.foo());
}

pub fn broken<F: Fn(&i32)>(x: &i32, f: F) {
    uncallable(x, |y| f(y));
}

fn main() { }
EOF

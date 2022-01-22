#!/usr/bin/env bash

rustc --crate-type lib -Zmir-opt-level=3 - 2>&1 << EOF

#![feature(trivial_bounds)]
trait Foo {
    fn test(self);
}
fn baz<T>()
where
    &'static str: Foo,
{
    "Foo".test()
}

EOF

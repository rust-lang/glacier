#!/bin/bash

rustc -Zverbose - <<'EOF'


#![allow(unused_parens)]
trait Foo {
    type Assoc;
}

fn called()
where
    for<'b> fn(&'b ()): Foo,
{
}

fn caller()
where
    (for<'a> fn(&'a ())): Foo,
{
    called()
}

fn main() {}
EOF


#/bin/bash

rustc --emit mir -Z mir-opt-level=2 - <<EOF
// build-pass
#![allow(dead_code)]
trait Foo {
    fn foo(&self);
}

fn foo<'a>(s: &'a mut ()) where &'a mut (): Foo {
    s.foo();
}
fn main() {}
EOF

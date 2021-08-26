#!/bin/bash

rustc -C link-dead-code - <<'EOF'
trait Yokeable<'a> {
    type Output: 'a;
}

fn project(_: for<'a> fn(<() as Yokeable<'a>>::Output)) {}

impl<'a> Yokeable<'a> for () {
    type Output = ();
}

fn crash() {
    project(|_| {});
}

fn main() {}
EOF

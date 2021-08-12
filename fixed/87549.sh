#!/usr/bin/env bash

rustc - -C incremental=true << 'EOF'
trait XTrait<A> {}

struct X<T: XTrait<A>, A> (T, A);

trait Y<'t> {
    type M;
    type N: 't;
}

impl<'t, T: XTrait<A>, A> Y<'t> for X<T, A> {
    type M = X<T, Self::N>;
    type N = &'t ();
}

fn main() {
    println!("Hello, world!");
}
EOF

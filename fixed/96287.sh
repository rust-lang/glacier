#!/bin/bash

cat > out.rs <<'EOF'
#![feature(type_alias_impl_trait)]

fn main() {}

trait TraitWithAssoc {
    type Assoc;
}

type Foo<V> = impl Trait<V::Assoc>; //~ associated type `Assoc` not found for `V`

trait Trait<U> {}

impl<W> Trait<W> for () {}

fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T> {
    ()
}

EOF

rustdoc --edition=2021 out.rs

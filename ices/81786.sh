#!/bin/sh

cat > out.rs <<'EOF'
#![feature(auto_traits, negative_impls)]

auto trait AutoTrait {}

pub struct Struct<K, V> {
    k: K,
    v: V
}

impl<T> !AutoTrait for *const T {}

impl<K, V> AutoTrait for Struct<K, V>
where
    K: AutoTrait,
    V: AutoTrait,
{
}

impl AutoTrait for Struct<usize, *const usize> {}

pub struct Wrap<'a> {
    inner: &'a Struct<usize, *const usize>,
}

fn main() {}
EOF

rustdoc out.rs -Z unstable-options --output-format json

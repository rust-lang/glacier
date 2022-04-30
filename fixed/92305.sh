#!/usr/bin/env bash

rustc --edition 2021 - << EOF

use std::iter;

fn f<T>(data: &[T]) -> impl Iterator<Item = Vec> {
    iter::empty()
}

fn g<T>(data: &[T], target: T) -> impl Iterator<Item = Vec<T>> {
    f(data).filter(|x| x == target)
}

fn main() {}

EOF

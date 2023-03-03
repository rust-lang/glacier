#!/bin/bash

rustc -Zvalidate-mir --crate-type=lib - <<'EOF'

#![feature(specialization)]

pub(crate) trait SpecTrait {
    type Assoc;
}

impl<C> SpecTrait for C {
    default type Assoc = Vec<Self>;
}

pub(crate) struct AssocWrap<C: SpecTrait> {
    _assoc: C::Assoc,
}

fn instantiate<C: SpecTrait>() -> AssocWrap<C> {
    loop {}
}

pub fn trigger() {
    instantiate::<()>();
}
EOF

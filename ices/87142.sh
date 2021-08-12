#!/usr/bin/env bash

rustc - --crate-type lib -C debuginfo=2 << 'EOF'
#![feature(type_alias_impl_trait, generator_trait, generators)]
use std::ops::{Generator};

pub trait GeneratorProviderAlt: Sized {
    type Gen: Generator<(), Return=(), Yield=()>;

    fn start(ctx: Context<Self>) -> Self::Gen;
}

pub struct Context<G: 'static + GeneratorProviderAlt> {
    // back-link to our generator state
    // In reality some other pointer type, but Box triggers the bug
    // Also in reality, points to a wrapper struct that only indirectly holds
    //   the generator state.
    link: Box<G::Gen>,
}

impl GeneratorProviderAlt for () {
    type Gen = impl Generator<(), Return=(), Yield=()>;
    fn start(ctx: Context<Self>) -> Self::Gen {
        move || {
            match ctx { _ => () } // make sure to use the context
            yield ();
        }
    }
}
EOF

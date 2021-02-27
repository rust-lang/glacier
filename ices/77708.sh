#!/bin/bash

rustc --edition 2018 -C incremental=foo --crate-type lib - <<'EOF'
#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]
use core::{convert::TryFrom, num::NonZeroUsize};

struct A<const N: NonZeroUsize>([u8; N.get()]) where [u8; N.get()]: Sized;

impl<'a, const N: NonZeroUsize> TryFrom<&'a [u8]> for A<N> where [u8; N.get()]: Sized {
    type Error = ();
    fn try_from(slice: &'a [u8]) -> Result<A<N>, ()> {
        let _x = <&[u8; N.get()] as TryFrom<&[u8]>>::try_from(slice);
        unimplemented!();
    }
}
EOF

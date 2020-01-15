#!/bin/bash

# https://github.com/rust-lang/rust/issues/44861
# Playground: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=bdd59503a0dc2d33c60a949fc7c57633
# You need to select `build` instead of `run`.
rustc --crate-type lib - << END

#![feature(specialization)]
#![feature(unsize, coerce_unsized)]

use std::ops::CoerceUnsized;
use std::marker::Unsize;

pub struct SmartassPtr<A: Smartass+?Sized>(A::Data);

pub trait Smartass {
    type Data;
    type Data2: CoerceUnsized<*const [u8]>;
}

pub trait MaybeObjectSafe {}

impl MaybeObjectSafe for () {}

impl<T> Smartass for T {
    type Data = <Self as Smartass>::Data2;
    default type Data2 = ();
}

impl Smartass for () {
    type Data2 = *const [u8; 1];
}

impl Smartass for MaybeObjectSafe {
    type Data = *const [u8];
    type Data2 = *const [u8; 0];
}

impl<U: Smartass+?Sized, T: Smartass+?Sized> CoerceUnsized<SmartassPtr<T>> for SmartassPtr<U>
    where <U as Smartass>::Data: std::ops::CoerceUnsized<<T as Smartass>::Data>
{}

pub fn conv(s: SmartassPtr<()>) -> SmartassPtr<MaybeObjectSafe> {
    s
}

END

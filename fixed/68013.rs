// https://github.com/rust-lang/rust/issues/68013
// Reduced from [https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=0f18f9d5d6071f9a10586e87e5dfd0b0]

#![feature(coerce_unsized)]

// These imports are also needed to get the ICE.
use std::rc::Rc;
use std::cell::Cell;
use std::ops::Deref;
use std::ops::CoerceUnsized;

#[derive(Clone)]
struct Redirectable<'a, T: ?Sized> {
    data: Rc<Cell<&'a T>>
}

impl<U, T: CoerceUnsized<U>> CoerceUnsized<Redirectable<'_, U>> for Redirectable<'_, T> {}

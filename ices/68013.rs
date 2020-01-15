#![feature(coerce_unsized)]

use std::rc::Rc;
use std::cell::Cell;
use std::ops::Deref;
use std::ops::CoerceUnsized;

#[derive(Clone)]
struct Redirectable<'a, T: ?Sized> {
    data: Rc<Cell<&'a T>>
}

impl<U, T: CoerceUnsized<U>> CoerceUnsized<Redirectable<'_, U>> for Redirectable<'_, T> {}

#![feature(coerce_unsized)]

use std::ops::CoerceUnsized;
use std::any::Any;

struct Foo<T> {
	data: Box<T>
}

impl<T> CoerceUnsized<Foo<dyn Any>> for Foo<T> {}

fn main() {}

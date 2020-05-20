#![feature(const_generics)]
#![crate_type = "lib"]

use core::marker::PhantomData;

struct Bug<'a, const S: &'a str>(PhantomData<&'a ()>);

impl Bug<'_, ""> {}

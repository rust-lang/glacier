#![feature(const_generics)]
use core::marker::PhantomData;

struct Bug<'a, const S: &'a str>(PhantomData<&'a ()>);

impl Bug<'_, {""}> {}

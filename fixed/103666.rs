#![feature(type_alias_impl_trait)]

type Tait<'b> = impl Sized;

fn foo(f: &dyn Fn(Tait)) {}

fn main() {}

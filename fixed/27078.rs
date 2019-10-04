#![feature(associated_consts)]
trait Foo { const BAR: i32; fn foo(self) -> &'static i32 { &<Self>::BAR }}
fn main() {}

#![feature(generic_associated_types)]

struct S;

trait Trait {
    type Associated<'a>;
}

impl Trait for S {
    type Associated<'a> = &'a ();
}

fn main() {}

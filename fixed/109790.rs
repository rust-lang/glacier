#![feature(inherent_associated_types)]

struct Foo<T>(T);

impl<'a> Foo<fn(&'a ())> {
    type Assoc = &'a ();
}

trait Other {}
impl Other for u32 {}

fn bar(_: for<'a> fn(Foo<fn(&'a ())>::Assoc)) {}


fn main() {}
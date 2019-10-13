#![crate_type = "lib"]

#![feature(generic_associated_types)]
struct Foo;

impl Iterator for Foo {
    type Item<'b> = &'b Foo;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#![feature(const_generics)]

trait A {}
struct B;
impl A for B {}

fn test<const T: &'static dyn A>() {
    unimplemented!()
}

fn main() {
    test::<{ &B }>();
}

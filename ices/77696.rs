#![feature(impl_trait_in_bindings)]

struct S;
trait Trait {
    fn demo(&self) {}
}
impl Trait for S {}
fn main() {
    let a: &impl Trait = &S;
}

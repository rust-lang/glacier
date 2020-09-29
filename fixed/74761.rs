#![feature(member_constraints)]
#![feature(type_alias_impl_trait)]

pub trait A {
    type B;
    fn f(&self) -> Self::B;
}
impl<'a, 'b> A for () {
    type B = impl core::fmt::Debug;

    fn f(&self) -> Self::B {}
}

fn main() {}

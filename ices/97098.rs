#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

pub trait Trait {
    type Assoc<'a>;
}

pub type Foo = impl for<'a> Trait<Assoc<'a> = FooAssoc<'a>>;
pub type FooAssoc<'a> = impl Sized;

struct Struct;
impl Trait for Struct {
    type Assoc<'a> = &'a u32;
}

const FOO: Foo = Struct;

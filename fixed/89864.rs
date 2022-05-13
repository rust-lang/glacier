#![crate_type = "lib"]
#![feature(fmt_internals)]

use core::{fmt, marker::PhantomData};

pub trait WrapperTrait {
    type Assoc;
}

pub trait Trait {
    type Assoc;
}

pub struct Ice<T>(<<Dynamic<T> as WrapperTrait>::Assoc as Trait>::Assoc);

fn ice<T>(i: Ice<T>)
where
    <<Dynamic<T> as WrapperTrait>::Assoc as Trait>::Assoc: fmt::Debug,
{
    fmt::ArgumentV1::new(&i.0, fmt::Debug::fmt);
}

pub struct TraitImplementor;

impl Trait for TraitImplementor {
    type Assoc = ConcreteAssoc;
}

pub struct ConcreteAssoc;

impl<T> WrapperTrait for Dynamic<T> {
    type Assoc = TraitImplementor;
}

pub struct Dynamic<T>(PhantomData<T>);

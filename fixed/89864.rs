#![crate_type = "lib"]

use core::{fmt, marker::PhantomData};

pub trait WrapperTrait {
    type Assoc;
}

pub trait Trait {
    type Assoc;
}

pub struct Ice<T>(<<Dynamic<T> as WrapperTrait>::Assoc as Trait>::Assoc);

impl<T> fmt::Debug for Ice<T>
where
    <<Dynamic<T> as WrapperTrait>::Assoc as Trait>::Assoc: fmt::Debug,
{
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        format_args!("{:?}", self.0);
        todo!()
    }
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

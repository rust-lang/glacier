use std::marker::PhantomData;

pub trait Filterable {
    type Filter;
}

pub trait ComponentMapper {
    type Component;
    fn entities_filtered(&self, <Self::Component as Filterable>::Filter)
        where Self::Component: Filterable;
}

pub struct VecMapper<T> {
    _marker: ::std::marker::PhantomData<T>
}

impl<T> VecMapper<T> {
    pub fn new() -> VecMapper<T> {
        VecMapper { _marker: PhantomData }
    }
}

impl<T> ComponentMapper for VecMapper<T> {
    type Component = T;
    fn entities_filtered(&self, _filter: T::Filter) where T: Filterable {}
}

fn main() { 
    let _m: &ComponentMapper<Component=()> = &VecMapper::<()>::new();
}

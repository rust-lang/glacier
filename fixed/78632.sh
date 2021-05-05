#!/bin/bash

rustc --test - << EOF
pub trait Corge<T> {
    type Fred;
}

impl Corge<u8> for () {
    type Fred = u32;
}

pub trait Waldo {
    type Quax;
}

impl Waldo for u32 {
    type Quax = u8;
}

pub trait Grault
where
    (): Corge<Self::Thud>,
{
    type Thud;
    fn bar(_: <() as Corge<Self::Thud>>::Fred) {}
}

impl<T> Grault for T
where
    T: Waldo,
    (): Corge<T::Quax>,
    <() as Corge<T::Quax>>::Fred: Waldo,
{
    type Thud = u8;
}

pub trait Plugh<I> {
    fn baz();
}

#[derive(Copy, Clone, Debug)]
pub struct Qiz<T> {
    foo: T,
}

impl<T> Plugh<<() as Corge<T::Thud>>::Fred> for Qiz<T>
where
    T: Grault,
    (): Corge<T::Thud>,
{
    fn baz() {}
}

#[cfg(test)]
mod tests {
    use super::{Grault, Plugh, Qiz};

    #[test]
    fn test1() {
        <u32 as Grault>::bar(0u32);
    }

    #[test]
    fn test2() {
        <Qiz<u32> as Plugh<u32>>::baz();
    }
}
EOF

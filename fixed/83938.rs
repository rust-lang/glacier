#![crate_type = "lib"]
#![feature(const_evaluatable_checked, const_generics, const_generics_defaults)]
#![allow(incomplete_features)]

type NpOne<T, const N: usize, const NP: usize = {N+1usize}> = [T; NP];

pub fn push<T, const N: usize>(_: [T; N], _: T)
    -> NpOne<T, N>
{
    todo!()
}

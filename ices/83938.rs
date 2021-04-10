#![crate_type = "lib"]
#![feature(const_generics_defaults)]

type NpOne<T, const N: usize, const NP: usize = {N+1usize}> = [T; NP];

pub fn push<T, const N: usize>(_: [T; N], _: T)
    -> NpOne<T, N>
{
    todo!()
}

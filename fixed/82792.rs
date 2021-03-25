#![crate_type = "lib"]
#![feature(const_generics_defaults)]

#[repr(C)]
pub struct Loaf<T: Sized, const N: usize = 1usize> {
    head: [T; N],
    slice: [T],
}

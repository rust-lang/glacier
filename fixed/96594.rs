#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![crate_type = "lib"]

#[repr(u8)]
pub enum SomeByte<const VALUE: u8>
where
    [(); VALUE as usize]:,
{
    Value = VALUE,
}

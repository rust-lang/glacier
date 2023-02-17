#![crate_type = "lib"]
#![feature(repr_simd)]

#[repr(simd)]
pub struct Simd([u8; 8]);

pub fn to_array_outer(x: Simd) -> [u8; 8] {
    let y = x;
    y.0
}

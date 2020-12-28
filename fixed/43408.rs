#![feature(const_fn)]
pub const fn sof<T:?Sized>() -> usize {
    10
}
fn to_byte_array<T>() -> [u8; sof::<T>()] {
     panic!()
}

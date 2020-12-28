#![feature(const_fn)]
pub const fn sof<T:?Sized>() -> usize {
    10
}

pub fn to_byte_array<T>() -> [u8; sof::<T>()] {
    panic!()
}

fn main() {}

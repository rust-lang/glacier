use std::ops::BitAnd;

trait Bits {
    fn bit<const I: u8>(self) -> bool;
}

impl<T> Bits for T where
    T: Copy + BitAnd<T, Output=T> + From<u8> + Eq
{
    fn bit<const I: usize>(self) -> bool {
        let i = 1 << I;
        let mask = T::from(i);
        mask & self == mask
    }
}

fn main() {
    let _ = 0xffu8.bit::<0>();
}

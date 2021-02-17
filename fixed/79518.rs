#![allow(incomplete_features)]
#![feature(const_generics, const_evaluatable_checked)]

use std::mem;

pub trait HardcodedPayload {
    type ArrayType: AsRef<[u8]> + AsMut<[u8]> + Default + PartialEq;
    const PAYLOAD: Self::ArrayType;

    fn test() where [u8; mem::size_of::<Self::ArrayType>()]: Sized + PartialEq { //workaround for `[u8; N]`
        let mut buf = [0u8; mem::size_of::<Self::ArrayType>()];
        //*insert read here*
        if buf == Self::PAYLOAD {} //crashed here
    }
}

pub struct UserStruct {
    _a: i32,
}

impl HardcodedPayload for UserStruct {
    type ArrayType = [u8; 2];
    const PAYLOAD: Self::ArrayType = [0xff, 0xff];
}

fn main() {}

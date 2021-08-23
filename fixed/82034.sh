#!/bin/bash

rustc --test -C incremental=foo - <<'EOF'
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![allow(incomplete_features)]
pub trait IsTrue {}
pub trait IsFalse {}

pub struct Assert<const CHECK: bool> {}

impl IsTrue for Assert<true> {}
impl IsFalse for Assert<false> {}

pub struct SliceConstWriter<'a, const N: usize> {
    ptr: &'a mut [u8]
}
impl<'a, const N: usize> SliceConstWriter<'a, {N}> {
    pub fn from_slice(vec: &'a mut [u8]) -> Self {
        Self {
            ptr: vec
        }
    }

    pub fn convert<const NN: usize>(mut self) -> SliceConstWriter<'a, {NN}> {
        SliceConstWriter {
            ptr: self.ptr
        }
    }
}

impl<'a, const N: usize> SliceConstWriter<'a, {N}> where Assert::<{N >= 2}>: IsTrue {
    // broken
    pub fn write_u8(mut self) -> SliceConstWriter<'a, {N-2}> {
        self.convert()
    }
    
    //working
    // pub fn write_u8(mut self) -> SliceConstWriter<'a, {N-2}> {
    //     SliceConstWriter {
    //         ptr: self.ptr
    //     }
    // }
}


#[cfg(test)]
mod tests {
    use crate::SliceConstWriter;

    #[test]
    fn it_works() {
        let mut buff = [0u8; 128];
        let mut a = SliceConstWriter::<10>::from_slice(&mut buff);

        let mut a = a.write_u8().write_u8().write_u8().write_u8().write_u8();
    }
}
EOF

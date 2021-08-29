#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

struct F<const S: &'static str>;
impl<const S: &'static str> X for F<{ S }> {
    const W: usize = 3;

    fn d(r: &[u8; Self::W]) -> F<{ S }> {
        let x: [u8; Self::W] = [0; Self::W];
        F
    }
}

pub trait X {
    const W: usize;
    fn d(r: &[u8; Self::W]) -> Self;
}

fn main() {}

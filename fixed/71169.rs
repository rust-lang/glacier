#![feature(const_generics)]
#![allow(incomplete_features)]

fn foo<const LEN: usize, const DATA: [u8; LEN]>() {}

fn main() {
    const DATA: [u8; 4] = *b"ABCD";
    foo::<4, DATA>();
}

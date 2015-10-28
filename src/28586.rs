#![feature(num_bits_bytes)]
use std::usize;
pub trait Foo {}
impl Foo for [u8; usize::BYTES] {}

fn main() {}

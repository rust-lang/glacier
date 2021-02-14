#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(const_generics)]

pub trait Buffer<const T: usize> {}

pub trait Device {
    type Buffer<const T: usize>: Buffer<{ T }>;

    fn create_buffer<const T: usize>(&self) -> Option<Self::Buffer<{ T }>>;
}

fn main() {}

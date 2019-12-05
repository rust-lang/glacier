#![feature(const_generics)]

trait HasSize {
    const SIZE: usize;
}

impl<const X: usize> HasSize for ArrayHolder<{X}> {
    const SIZE: usize = X;
}

struct ArrayHolder<const X: usize>([u32; X]);

impl<const X: usize> ArrayHolder<{X}> {
    pub const fn new() -> Self {
        ArrayHolder([0; Self::SIZE])
    }
}

fn main() {}

#![feature(const_generics)]

struct Stack<const N: usize> {
    stack: [u64; N / 8],
}

fn main() {}

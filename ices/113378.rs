#![feature(const_trait_impl)]
#![feature(effects)]

trait Value {}

impl<T> const Value for T {
    const fn value() -> u32 {
        0
    }
}

fn main() {}

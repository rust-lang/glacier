#![feature(effects)]

struct Bar<T>(T);

impl<T> Bar<T> {
    const fn value() -> usize {
        42
    }
}

struct Foo<const N: [u8; Bar::<u32>::value()]>;

fn main() {}

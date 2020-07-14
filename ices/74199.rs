#![feature(const_generics)]

struct Foo<
    const N: [u8; {
        struct Foo<const N: usize>;

        impl<const N: usize> Foo<N> {
            fn value() -> usize {
                N
            }
        }

        Foo::<17>::value()
    }],
>;

fn main() {}

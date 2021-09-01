#![allow(incomplete_features)]
#![feature(const_generics)]

trait Bar<const N: usize> {}

trait Foo<'a> {
    const N: usize;
    type Baz: Bar<{ Self::N }>;
}

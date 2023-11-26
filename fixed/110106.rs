#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct D<T> {
  a: T
}

impl<T: Default> D<T> {
    type Item = T;

    fn next() -> Self::Item {
        Self::Item::default()
    }
}


fn main() {
}


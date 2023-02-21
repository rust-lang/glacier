#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct S<T>(T);

impl<T, 'a> S<T> { // Also repros with any other lifetimes such as '_  ,switching order to 'a, T also repros.
    type P = T;
}

fn main() {
    type A = S<()>::P;
}

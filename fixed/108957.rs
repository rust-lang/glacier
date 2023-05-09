#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Choose<T>(T);

impl<T: Copy> Choose<T> {
    type Result = Vec<T>;
}

fn main() {
    let _: Choose<&str>::Result = vec!["..."];
}

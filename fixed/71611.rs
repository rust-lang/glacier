#![feature(const_generics)]
#![feature(const_compare_raw_pointers)]

fn func<A, const F: fn(inner: A)>(outer: A) {
    F(outer);
}

fn main() {}

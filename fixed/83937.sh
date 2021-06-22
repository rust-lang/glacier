#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "temp"
version = "0.1.0"
edition = "2018"

[[bench]]
name = "counting_sort"
EOF

mkdir -p benches src

cat > src/lib.rs <<'EOF'
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

pub trait UsizeConversions {
    fn into_usize(self) -> usize;
}

impl const UsizeConversions for u8 {
    fn into_usize(self) -> usize {
        self as usize
    }
}

pub trait CountingSort {
    fn sort(&mut self);
}

impl CountingSort for [u8] {
    #[inline] // when this is removed, everything works well
    fn sort(&mut self) {
        let _counts = [0usize; u8::MAX.into_usize() + 1];
    }
}

#[test]
fn works_in_tests() {
    let mut data = vec![5, 2, 8, 10];
    CountingSort::sort(&mut *data);
}
EOF

cat > benches/counting_sort.rs <<'EOF'
#![feature(test)]

extern crate test;

use temp::CountingSort;
use test::Bencher;

#[bench]
fn counting_sort(b: &mut Bencher) {
    b.iter(|| {
        let mut data = vec![5, 2, 8, 10]; 
        CountingSort::sort(&mut *data);
    });
}
EOF

cargo bench

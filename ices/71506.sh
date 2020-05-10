#!/bin/bash

tmp="$(mktemp -d)"

if [[ ! $tmp || ! -d $tmp ]]
then
    echo "Could not create temporary directory"
    exit 1
fi

cleanup() {
    rm -r "$tmp"
}

trap cleanup EXIT

cd "$tmp"

cargo new bug-rust-71506 && cd bug-rust-71506

cat > Cargo.toml <<EOF

[package]
name = "bug-rust-71506"
version = "0.1.0"
authors = ["Mathieu Poumeyrol <kali@zoy.org>"]
edition = "2018"

[workspace]
members = ["lib"]

[dependencies]
lib = { path = "lib" }

[profile.release]
lto = true
EOF

rm -f main.rs

mkdir -p lib/src

cat > lib/Cargo.toml <<EOF
[package]
name = "lib"
version = "0.1.0"
authors = ["Mathieu Poumeyrol <kali@zoy.org>"]
edition = "2018"

[dependencies]
ndarray = "=0.13.1"
EOF

cat > lib/src/lib.rs <<EOF
#![allow(deprecated)]

use ndarray::*;

#[derive(Debug, Clone, Default)]
pub struct GlobalLpPool {
    p: usize,
}

impl GlobalLpPool {
    pub fn eval(&self, input: Vec<f64>) -> Vec<f64> {
        let input = ndarray::Array::from_vec(input);

        // second call to into_shape() is redundant, but the bug only triggers
        // if it is in there
        let input = input.into_shape((2, 1)).unwrap();
        let input = input.into_shape((2, 1)).unwrap();

        let result = if self.p == 1 {
            unimplemented!("need the if to trigger the bug");
        } else {
            input
                .fold_axis(Axis(0), 0.0, |&a, &b| a + b.powi(self.p as i32))
                .map(|a| a.powf((self.p as f64).recip()))
        };
        result.to_vec()
    }
}
EOF

cat > src/main.rs <<EOF
fn main() {
    let op = lib::GlobalLpPool::default();
    let result = op.eval(vec![1.0f64, 2.0]);
    println!("{:?}", result);
}
EOF

cd ..

cargo build --release

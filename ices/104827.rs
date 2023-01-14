#![feature(try_trait_v2)]

use std::ops::FromResidual;

struct MySnafu;

fn test_function() {
    impl FromResidual for MySnafu {
        fn from_residual(s: Self) -> Self {
            todo!()
        }
    }
}

fn main() {}

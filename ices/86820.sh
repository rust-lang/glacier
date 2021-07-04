#!/bin/bash

cat > Cargo.toml <<'EOF'
[package]
name = "abc"
version = "0.0.1"
edition = "2018"
EOF

mkdir -p src

cat > src/lib.rs <<'EOF'
use std::ops::BitAnd;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_set() {
        assert!(0xffu8.bit::<0>());
    }
}

trait Bits {
    fn bit<const I: u8>(self) -> bool;
}

impl<T> Bits for T where
    T: Copy + BitAnd<T, Output=T> + From<u8> + Eq
{
    fn bit<const I: usize>(self) -> bool {
        let i = 1 << I;
        let mask = T::from(i);
        mask & self == mask
    }
}
EOF

cargo test

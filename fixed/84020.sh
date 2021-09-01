#!/bin/bash

rustc --crate-name bug --edition=2018 --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no --test -C metadata=1974b6bb1d213572 -C extra-filename=-1974b6bb1d213572 --out-dir /tmp --target aarch64-apple-darwin - <<'EOF'
#![feature(const_generics, platform_intrinsics, repr_simd)]
#![allow(incomplete_features)]

extern "platform-intrinsic" {
    fn simd_eq<T, U>(a: T, b: T) -> U;
    fn simd_and<T>(a: T, b: T) -> T;
}

#[derive(Copy, Clone)]
#[repr(simd)]
pub struct F<const N: usize>([f32; N]);

impl<const N: usize> F<N> {
    pub fn splat(value: f32) -> Self {
        Self([value; N])
    }

    pub fn eq(self, other: Self) -> I<N> {
        unsafe { simd_eq(self, other) }
    }
}

#[repr(simd)]
pub struct I<const N: usize>([i32; N]);

impl<const N: usize> core::ops::BitAnd for I<N> {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        unsafe { simd_and(self, other) }
    }
}

impl<const N: usize> I<N> {
    pub fn to_array(self) -> [bool; N] {
        let mut array = [false; N];
        let mut i = 0;
        while i < N {
            array[i] = self.0[i] == -1;
            i += 1;
        }
        array
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn bug() {
        let a = F::<64>::splat(0.);
        let b = F::<64>::splat(0.);
        let c = F::<64>::splat(0.);
        let m = a.eq(b) & a.eq(c);
        assert!(m.to_array()[0]);
    }
}
EOF

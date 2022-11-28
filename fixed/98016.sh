#!/bin/bash

rustc --edition=2021 -Zmir-opt-level=3 -Zvalidate-mir - <<EOF

#![crate_type = "lib"]
#![feature(portable_simd)]

use std::simd::Simd;
const N: usize = 8;

#[no_mangle]
// CHECK-LABEL: @wider_reduce_into_iter
pub fn wider_reduce_into_iter(x: Simd<u8, N>) -> u16 {
    // CHECK: zext <8 x i8>
    // CHECK-SAME: to <8 x i16>
    // CHECK: call i16 @llvm.vector.reduce.add.v8i16(<8 x i16>
    x.to_array().into_iter().map(u16::from).sum()
}

EOF

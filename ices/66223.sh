#!/bin/bash

rustc -C opt-level=3 - << 'END'
#![feature(llvm_asm)]

unsafe fn _mm512_set1_epi64(a: i64) {
    llvm_asm! {
        "vpbroadcastq zmm0{k1}, $0"
        :: "m"(a)
        :: "intel"
    }
}

fn main() {
    unsafe { _mm512_set1_epi64(123); }
}
END

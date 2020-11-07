#!/bin/bash

rustc --crate-type=lib -Z mir-opt-level=1 -Z new-llvm-pass-manager=yes -Z unsound-mir-opts=yes -Z verify-llvm-ir=yes -Z validate-mir=yes -Z polonius=yes -Z polymorphize=yes -C debuginfo=2 -C opt-level=1 - << EOF
#![feature(no_core, lang_items)]
#![no_core]

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

#[no_mangle]
fn test() {
    &1;
}

EOF

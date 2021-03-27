#![feature(core_intrinsics, rustc_attrs)]
use std::intrinsics::rustc_peek;
struct S;
#[rustc_mir(rustc_peek_definite_init)]
fn foo(z: S) {
    let ret;
    rustc_peek(rustc_peek(z));
    ret
}

fn main() {}

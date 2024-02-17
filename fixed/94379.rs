#![crate_type = "staticlib"]
#![feature(core_intrinsics)]
use std::intrinsics::rustc_peek;
#[no_mangle]
fn foo() -> i32 {
    let x = 0;
    rustc_peek(x)
}

#![crate_type = "staticlib"]
#![feature(rustc_attrs)]
struct S;
#[rustc_mir()]
fn foo(x: &S) {
    let ret;
    *x;
    ret
}

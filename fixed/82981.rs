#![crate_type = "staticlib"]
#![feature(rustc_attrs)]
#[rustc_layout_scalar_valid_range_start(A)]
struct T {}
const C: T = T {};

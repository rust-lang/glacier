#![feature(rustc_attrs)]
#[rustc_mir(borrowck_graphviz_postflow="hello.dot")]
fn main() {}

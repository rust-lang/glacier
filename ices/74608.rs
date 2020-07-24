#![feature(rustc_attrs)]

#[rustc_args_required_const(1)]
fn foo(_: u8) {}

fn main() {
    foo(1);
}

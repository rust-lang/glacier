use std::mem;
fn main() {
    let foo: unsafe extern "rust-intrinsic" fn(int) -> uint = mem::transmute;
}

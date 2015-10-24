use std::mem;
fn main() {
    let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = mem::transmute;
}

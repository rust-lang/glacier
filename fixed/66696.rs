// This is ICE deliberately.

#![feature(unboxed_closures)]
fn main() {
    unsafe { std::mem::transmute::<usize, extern "rust-call" fn()>(5); }
}

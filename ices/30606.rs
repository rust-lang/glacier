#![feature(llvm_asm)]

fn f() {}

fn main() {
    unsafe {llvm_asm!( "" :: "r"(f))}
}

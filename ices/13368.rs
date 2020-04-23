#![feature(llvm_asm)]

fn main() {
    let arr: [u8; 16];
    unsafe { llvm_asm!("" : "=m"(arr)); }
    println!("{:?}", arr);
}

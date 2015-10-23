#![feature(asm)]

fn main() {
    let arr: [u8; 16];
    unsafe { asm!("" : "=m"(arr)); }
    println!("{:?}", arr);
}

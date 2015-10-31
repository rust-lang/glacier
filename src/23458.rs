#![feature(asm)]

fn main() {
    unsafe {
        asm!("int $3");
    } 
}

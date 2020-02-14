#![feature(asm)]

fn main() {
    unsafe { asm!(".ascii \"Xen\0\""); }
}

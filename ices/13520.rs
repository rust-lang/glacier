#![feature(llvm_asm)]

fn main() {
    let x = ();
    unsafe {
        let p: *const () = &x;
        llvm_asm!("" :: "r"(*p));
    }
}

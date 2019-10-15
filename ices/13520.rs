#![feature(asm)]

fn main() {
    let x = ();
    unsafe {
        let p: *const () = &x;
        asm!("" :: "r"(*p));
    }
}

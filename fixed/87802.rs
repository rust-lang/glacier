#![feature(asm)]

fn hmm() -> ! {
    let x;
    unsafe {
        asm!("/* {0} */", out(reg) x);
    }
    x
}

fn main() {
    hmm();
}

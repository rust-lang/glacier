#![feature(asm)]

extern "C" fn foo() { }

fn main() {
    let x: usize;
    unsafe {
        asm!("movq $1, $0" : "=r"(x) : "r"(foo));
    }
    assert!(x != 0);
}

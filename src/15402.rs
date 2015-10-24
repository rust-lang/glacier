#![feature(asm)]

pub struct Wrapper(uint);

fn main() {
    let mut value: Wrapper = Wrapper(7);
    unsafe {
        asm!("mov %eax, $0" : "+r"(value));
    }
}

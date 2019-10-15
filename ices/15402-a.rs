#![feature(asm)]

pub struct Wrapper(u32);

fn main() {
    let mut _value: Wrapper = Wrapper(7);
    unsafe {
        asm!("mov %eax, $0" : "+r"(_value));
    }
}
